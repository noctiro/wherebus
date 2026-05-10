use std::sync::Arc;
use std::time::Duration;

use super::cache::{Freshness, TypedCache};
use super::connectivity::{ConnectivityTracker, NetState};
use super::storage::Storage;
use super::storage::tables;
use crate::models::*;
use crate::domain::{
    CacheCategory, CacheStats, LineDetailSnapshot, NearbyGroup, NearbyLineView,
    NearbySnapshot, NearbyStationView, NetworkState, ServiceInfo,
};
use crate::providers::{self, BusDataProvider, ProviderError};
use redb::TableDefinition;

const STATIONS_TTL: Duration = Duration::from_secs(300);
const LINES_TTL: Duration = Duration::from_secs(30);
const DETAIL_TTL: Duration = Duration::from_secs(3600);
const ALL_LINES_TTL: Duration = Duration::from_secs(86400);

const STATIONS_CAP: usize = 50;
const LINES_CAP: usize = 100;
const DETAIL_CAP: usize = 30;
const ALL_LINES_CAP: usize = 3;

pub struct RuntimeFacade {
    inner: Arc<dyn BusDataProvider>,
    stations: Arc<TypedCache<Vec<Station>>>,
    lines: Arc<TypedCache<Vec<LineSummary>>>,
    detail: Arc<TypedCache<LineDetail>>,
    all_lines: Arc<TypedCache<Vec<BusRoute>>>,
    pub connectivity: Arc<ConnectivityTracker>,
    storage: Arc<Storage>,
}

impl RuntimeFacade {
    pub fn open() -> anyhow::Result<Self> {
        let storage = Arc::new(Storage::open()?);
        let config = storage.load_config().unwrap_or_default();
        let provider = providers::create_provider(&config.service_id);
        Ok(Self {
            inner: provider,
            stations: Arc::new(TypedCache::new(STATIONS_TTL, STATIONS_CAP)),
            lines: Arc::new(TypedCache::new(LINES_TTL, LINES_CAP)),
            detail: Arc::new(TypedCache::new(DETAIL_TTL, DETAIL_CAP)),
            all_lines: Arc::new(TypedCache::new(ALL_LINES_TTL, ALL_LINES_CAP)),
            connectivity: Arc::new(ConnectivityTracker::new()),
            storage,
        })
    }

    pub fn switch_provider(&self, service_id: &str) -> Arc<Self> {
        let provider = providers::create_provider(service_id);
        Arc::new(Self {
            inner: provider,
            stations: Arc::new(TypedCache::new(STATIONS_TTL, STATIONS_CAP)),
            lines: Arc::new(TypedCache::new(LINES_TTL, LINES_CAP)),
            detail: Arc::new(TypedCache::new(DETAIL_TTL, DETAIL_CAP)),
            all_lines: Arc::new(TypedCache::new(ALL_LINES_TTL, ALL_LINES_CAP)),
            connectivity: self.connectivity.clone(),
            storage: self.storage.clone(),
        })
    }

    pub fn net_state(&self) -> NetState {
        self.connectivity.state()
    }

    pub fn available_services(&self) -> Vec<ServiceInfo> {
        providers::available_services()
            .into_iter()
            .map(|service| ServiceInfo {
                id: service.id.to_string(),
                city_name: service.city.name().to_string(),
                province: service.city.province().to_string(),
                provider_name: service.provider.to_string(),
            })
            .collect()
    }

    pub fn current_service(&self) -> ServiceInfo {
        let config = self.load_config().unwrap_or_default();
        self.available_services()
            .into_iter()
            .find(|service| service.id == config.service_id)
            .unwrap_or_else(|| ServiceInfo {
                id: config.service_id,
                city_name: self.city_name().to_string(),
                province: String::new(),
                provider_name: self.provider_name().to_string(),
            })
    }

    pub fn provider_name(&self) -> &str {
        self.inner.provider_name()
    }

    pub fn city_name(&self) -> &str {
        self.inner.city_name()
    }

    pub fn clear(&self) {
        self.stations.clear();
        self.lines.clear();
        self.detail.clear();
        self.all_lines.clear();
        let _ = self.storage.clear_cache();
    }

    pub fn cache_stats(&self) -> CacheStats {
        CacheStats {
            stations: self.storage.table_entry_count(tables::STATIONS_TABLE),
            station_lines: self.storage.table_entry_count(tables::STATION_LINES_TABLE),
            line_detail: self.storage.table_entry_count(tables::LINE_DETAIL_TABLE),
            all_lines: self.storage.table_entry_count(tables::ALL_LINES_TABLE),
        }
    }

    pub fn clear_category(&self, category: CacheCategory) {
        match category {
            CacheCategory::Stations => {
                self.stations.clear();
                let _ = self.storage.clear_table(tables::STATIONS_TABLE);
            }
            CacheCategory::StationLines => {
                self.lines.clear();
                let _ = self.storage.clear_table(tables::STATION_LINES_TABLE);
            }
            CacheCategory::LineDetail => {
                self.detail.clear();
                let _ = self.storage.clear_table(tables::LINE_DETAIL_TABLE);
            }
            CacheCategory::AllLines => {
                self.all_lines.clear();
                let _ = self.storage.clear_table(tables::ALL_LINES_TABLE);
            }
        }
    }

    pub fn save_config(&self, config: &crate::models::AppConfig) -> anyhow::Result<()> {
        self.storage.save_config(config)?;
        Ok(())
    }

    pub fn load_config(&self) -> anyhow::Result<crate::models::AppConfig> {
        Ok(self.storage.load_config().unwrap_or_default())
    }

    pub fn network_state(&self) -> NetworkState {
        match self.net_state() {
            NetState::Online => NetworkState::Online,
            NetState::Degraded => NetworkState::Degraded,
            NetState::Offline => NetworkState::Offline,
        }
    }

    pub fn clear_cache(&self) -> anyhow::Result<()> {
        self.clear();
        Ok(())
    }

    pub fn clear_cache_category(
        &self,
        category: CacheCategory,
    ) -> anyhow::Result<()> {
        self.clear_category(category);
        Ok(())
    }

    pub async fn nearby_snapshot(
        &self,
        lat: f64,
        lng: f64,
    ) -> Result<NearbySnapshot, String> {
        let stations = self
            .nearby_stations(lat, lng)
            .await
            .map_err(|e| e.to_string())?;

        tracing::debug!("[nearby] found {} stations", stations.len());
        let stations_to_fetch: Vec<_> = stations.into_iter().take(5).collect();
        let futs: Vec<_> = stations_to_fetch
            .iter()
            .map(|station| self.station_lines(&station.name, station.lat, station.lng))
            .collect();
        let results = futures::future::join_all(futs).await;

        let all_routes = self.all_lines.get(&"all".to_string());

        let mut groups = Vec::new();
        for (station, result) in stations_to_fetch.into_iter().zip(results) {
            let mut lines = match result {
                Ok(l) if !l.is_empty() => l,
                Ok(_) => {
                    tracing::debug!("[nearby] {} → 0 lines, skip", station.name);
                    continue;
                }
                Err(e) => {
                    tracing::debug!("[nearby] {} → error: {}", station.name, e);
                    continue;
                }
            };

            if let Some(ref cached_routes) = all_routes {
                for line in lines.iter_mut() {
                    if line.endpoints.origin.is_empty() {
                        if let Some(route) = cached_routes.data.iter().find(|r| r.direction_id == line.direction_id) {
                            line.endpoints = route.endpoints.clone();
                        }
                    }
                }
            }

            tracing::debug!(
                "[nearby] {} → {} lines",
                station.name,
                lines.len()
            );

            lines.sort_by(|a, b| {
                let a_running = a.run_state == RunState::Running;
                let b_running = b.run_state == RunState::Running;
                b_running.cmp(&a_running).then(
                    a.arrival
                        .proximity_score()
                        .cmp(&b.arrival.proximity_score()),
                )
            });

            for line in lines.iter().take(3) {
                tracing::debug!(
                    "[nearby]   sorted: {} {:?} score={}",
                    line.name, line.arrival, line.arrival.proximity_score()
                );
            }

            groups.push(NearbyGroup {
                station: NearbyStationView {
                    station_name: station.name,
                    distance_m: station.distance_m as i32,
                    line_count: lines.len() as i32,
                },
                lines: lines
                    .into_iter()
                    .map(|line| {
                        let (stations_away, minutes_away, distance_m) = match &line.arrival {
                            ArrivalEstimate::Arriving => (0, 0, 0),
                            ArrivalEstimate::Approaching {
                                stations_away,
                                minutes_away,
                                distance_m,
                            } => (
                                *stations_away as i32,
                                minutes_away.map(|v| v as i32).unwrap_or(-1),
                                distance_m.map(|v| v as i32).unwrap_or(0),
                            ),
                            _ => (0, -1, 0),
                        };
                        NearbyLineView {
                            line_name: line.name,
                            direction_id: line.direction_id,
                            endpoints: line.endpoints,
                            arrival_state: line.arrival.state_tag().to_string(),
                            run_state: Some(line.run_state),
                            stations_away,
                            minutes_away,
                            distance_m,
                        }
                    })
                    .collect(),
            });
        }

        Ok(NearbySnapshot { groups })
    }

    pub async fn refresh_nearby(
        &self,
        lat: f64,
        lng: f64,
    ) -> Result<NearbySnapshot, String> {
        tracing::debug!("[nearby] refresh_nearby: invalidating caches");
        self.stations.invalidate(&format!("{:.4},{:.4}", lat, lng));
        self.lines.clear();
        self.nearby_snapshot(lat, lng).await
    }

    pub async fn line_detail_snapshot(
        &self,
        direction_id: &str,
        target_order: u32,
    ) -> Result<LineDetailSnapshot, String> {
        let detail = self
            .line_detail(direction_id)
            .await
            .map_err(|e| e.to_string())?;
        let realtime_target_order = if target_order > 0 {
            target_order
        } else {
            detail
                .topology
                .stations
                .first()
                .map(|station| station.order)
                .unwrap_or(1)
        };
        let realtime = self
            .realtime(direction_id, realtime_target_order)
            .await
            .ok();
        Ok(LineDetailSnapshot { detail, realtime })
    }

    pub async fn nearby_stations(&self, lat: f64, lng: f64) -> Result<Vec<Station>, ProviderError> {
        let key = format!("{:.4},{:.4}", lat, lng);

        if let Some(cached) = self.stations.get(&key) {
            if cached.freshness == Freshness::Fresh && !self.stations.needs_refresh(&key) {
                tracing::debug!("[cache] stations hit (fresh): {}", key);
                return Ok(cached.data);
            }
            tracing::debug!("[cache] stations hit (stale, bg refresh): {}", key);
            let inner = self.inner.clone();
            let conn = self.connectivity.clone();
            self.bg_refresh(
                self.stations.clone(),
                key.clone(),
                tables::STATIONS_TABLE,
                move || async move {
                    let r = inner.nearby_stations(lat, lng).await;
                    conn.record(r.is_ok());
                    r
                },
            );
            return Ok(cached.data);
        }

        tracing::debug!("[cache] stations miss: {}", key);
        self.fetch_or_wait(
            &self.stations,
            &key,
            tables::STATIONS_TABLE,
            STATIONS_TTL,
            || self.inner.nearby_stations(lat, lng),
        )
        .await
    }

    pub async fn station_lines(
        &self,
        station: &str,
        lat: f64,
        lng: f64,
    ) -> Result<Vec<LineSummary>, ProviderError> {
        let key = station.to_string();

        if let Some(cached) = self.lines.get(&key) {
            if cached.freshness == Freshness::Fresh && !self.lines.needs_refresh(&key) {
                tracing::debug!("[cache] lines hit (fresh): {}", key);
                return Ok(cached.data);
            }
            tracing::debug!("[cache] lines hit (stale, bg refresh): {}", key);
            let inner = self.inner.clone();
            let conn = self.connectivity.clone();
            let s = station.to_string();
            self.bg_refresh(
                self.lines.clone(),
                key.clone(),
                tables::STATION_LINES_TABLE,
                move || async move {
                    let r = inner.station_lines(&s, lat, lng).await;
                    conn.record(r.is_ok());
                    r
                },
            );
            return Ok(cached.data);
        }

        tracing::debug!("[cache] lines miss: {}", key);
        self.fetch_or_wait(
            &self.lines,
            &key,
            tables::STATION_LINES_TABLE,
            LINES_TTL,
            || self.inner.station_lines(station, lat, lng),
        )
        .await
    }

    pub async fn line_detail(
        &self,
        direction_id: &str,
    ) -> Result<LineDetail, ProviderError> {
        if let Some(cached) = self.detail.get(direction_id) {
            if cached.freshness == Freshness::Fresh && !self.detail.needs_refresh(direction_id) {
                return Ok(cached.data);
            }
            let inner = self.inner.clone();
            let conn = self.connectivity.clone();
            let k = direction_id.to_string();
            self.bg_refresh(
                self.detail.clone(),
                direction_id.to_string(),
                tables::LINE_DETAIL_TABLE,
                move || async move {
                    let r = inner.line_detail(&k).await;
                    conn.record(r.is_ok());
                    r
                },
            );
            return Ok(cached.data);
        }

        let k = direction_id.to_string();
        self.fetch_or_wait(
            &self.detail,
            &k,
            tables::LINE_DETAIL_TABLE,
            DETAIL_TTL,
            || self.inner.line_detail(direction_id),
        )
        .await
    }

    pub async fn realtime(
        &self,
        direction_id: &str,
        order: u32,
    ) -> Result<RealTimeData, ProviderError> {
        if !self.connectivity.should_attempt() {
            return Err(ProviderError::Network("离线".to_string()));
        }
        let result = self.inner.realtime(direction_id, order).await;
        self.connectivity.record(result.is_ok());
        result
    }

    pub async fn all_lines(&self) -> Result<Vec<BusRoute>, ProviderError> {
        let key = "all".to_string();

        if let Some(cached) = self.all_lines.get(&key) {
            if cached.freshness == Freshness::Fresh && !self.all_lines.needs_refresh(&key) {
                return Ok(cached.data);
            }
            let inner = self.inner.clone();
            let conn = self.connectivity.clone();
            self.bg_refresh(
                self.all_lines.clone(),
                key.clone(),
                tables::ALL_LINES_TABLE,
                move || async move {
                    let r = inner.all_lines().await;
                    conn.record(r.is_ok());
                    r
                },
            );
            return Ok(cached.data);
        }

        self.fetch_or_wait(
            &self.all_lines,
            &key,
            tables::ALL_LINES_TABLE,
            ALL_LINES_TTL,
            || self.inner.all_lines(),
        )
        .await
    }

    async fn fetch_or_wait<T, F, Fut>(
        &self,
        cache: &Arc<TypedCache<T>>,
        key: &str,
        table: TableDefinition<'static, &str, &[u8]>,
        ttl: Duration,
        fetch: F,
    ) -> Result<T, ProviderError>
    where
        T: Clone + Send + serde::Serialize + serde::de::DeserializeOwned + 'static,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, ProviderError>>,
    {
        // Check redb (fresh) before network
        if let Some(data) = self.storage.get_cached::<T>(table, key, ttl) {
            cache.insert(key.to_string(), data.clone());
            return Ok(data);
        }

        if let Some(notify) = cache.register_pending(key) {
            notify.notified().await;
            if let Some(cached) = cache.get(key) {
                return Ok(cached.data);
            }
            return Err(ProviderError::Network("请求失败".to_string()));
        }

        if !self.connectivity.should_attempt() {
            cache.complete_pending(key);
            // Offline fallback: serve stale from redb
            if let Some(stale) = self.storage.get_stale::<T>(table, key) {
                cache.insert(key.to_string(), stale.clone());
                return Ok(stale);
            }
            return Err(ProviderError::Network("离线".to_string()));
        }

        let result = (fetch)().await;
        self.connectivity.record(result.is_ok());

        match result {
            Ok(data) => {
                cache.insert(key.to_string(), data.clone());
                // Write-through to redb (async)
                let storage = self.storage.clone();
                let k = key.to_string();
                let d = data.clone();
                tokio::spawn(async move {
                    let _ = storage.put_cached(table, &k, &d);
                });
                cache.complete_pending(key);
                Ok(data)
            }
            Err(e) => {
                cache.complete_pending(key);
                // Serve stale on network error
                if let Some(stale) = self.storage.get_stale::<T>(table, key) {
                    cache.insert(key.to_string(), stale.clone());
                    return Ok(stale);
                }
                Err(e)
            }
        }
    }

    fn bg_refresh<T, F, Fut>(
        &self,
        cache: Arc<TypedCache<T>>,
        key: String,
        table: TableDefinition<'static, &str, &[u8]>,
        fetch: F,
    ) where
        T: Clone + Send + serde::Serialize + serde::de::DeserializeOwned + 'static,
        F: FnOnce() -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<T, ProviderError>> + Send,
    {
        if cache.register_pending(&key).is_some() {
            return;
        }
        let storage = self.storage.clone();
        tokio::spawn(async move {
            if let Ok(data) = (fetch)().await {
                let _ = storage.put_cached(table, &key, &data);
                cache.insert(key.clone(), data);
            }
            cache.complete_pending(&key);
        });
    }
}
