use std::collections::{BTreeMap, HashMap, VecDeque};
use std::sync::{Mutex, OnceLock};

use crux_core::Core;
use crux_core::bridge::{Bridge as CruxBridge, EffectId, JsonFfiFormat};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::runtime::{Builder, Runtime};

use crate::app::WherebusApp;
use crate::app::dto::{BootstrapData, NearbyData, ServiceSwitchData};
use crate::app::effect::{
    ClearCacheCategoryOp, FetchDetailOp, FetchNearbyOp, SaveConfigOp, SwitchServiceOp,
};
use crate::app::view::{
    CacheStatsView, CityItemView, NearbyItemView,
};
use crate::domain::{
    ALL_CITIES, AppConfig, ArrivalEstimate, BusRoute, LineSummary, NetworkState,
    RunState, ServiceInfo,
};
use crate::kernel::storage::platform::set_android_data_dir;
use crate::service::{CacheCategory, ManageService, QueryService, Services};

struct NativeEngine {
    runtime: Runtime,
    services: Services,
}

struct SharedCoreBridge {
    bridge: Mutex<CruxBridge<WherebusApp, JsonFfiFormat>>,
}

enum InternalEffectResult {
    Resolve(String),
    NotifyOnly,
    Platform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct BridgeRequest {
    id: u32,
    effect: Value,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct BridgeEnvelope {
    ok: bool,
    data: Option<String>,
    error: Option<String>,
}

impl NativeEngine {
    fn shared() -> Result<&'static Self, String> {
        static ENGINE: OnceLock<Result<NativeEngine, String>> = OnceLock::new();

        ENGINE
            .get_or_init(|| {
                let runtime = Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .map_err(|err| err.to_string())?;
                let services = Services::open().map_err(|err| err.to_string())?;
                Ok(NativeEngine { runtime, services })
            })
            .as_ref()
            .map_err(Clone::clone)
    }

    fn bootstrap_data(&self) -> Result<BootstrapData, String> {
        self.runtime.block_on(async {
            let config = self
                .services
                .load_config()
                .await
                .map_err(|err| err.to_string())?;
            let current = self.services.current_service().await;
            let services = self.services.available_services().await;
            let stats = self
                .services
                .cache_stats()
                .await
                .map_err(|err| err.to_string())?;

            Ok(BootstrapData {
                config,
                provider_name: current.provider_name.clone(),
                city_label: format!("{} - {}", current.city_name, current.provider_name),
                location_permission: false,
                city_picker_cities: build_city_picker_items(services),
                cache_stats: CacheStatsView {
                    stations: stats.stations,
                    station_lines: stats.station_lines,
                    line_detail: stats.line_detail,
                    all_lines: stats.all_lines,
                },
            })
        })
    }

    fn save_config(&self, config: AppConfig) -> Result<(), String> {
        self.runtime
            .block_on(self.services.save_config(config))
            .map_err(|err| err.to_string())
    }

    fn switch_service(&self, service_id: String) -> Result<ServiceSwitchData, String> {
        self.runtime.block_on(async {
            let service = self
                .services
                .switch_service(&service_id)
                .await
                .map_err(|err| err.to_string())?;
            Ok(ServiceSwitchData {
                provider_name: service.provider_name.clone(),
                city_label: format!("{} - {}", service.city_name, service.provider_name),
            })
        })
    }

    fn cache_stats(&self) -> Result<CacheStatsView, String> {
        self.runtime.block_on(async {
            let stats = self
                .services
                .cache_stats()
                .await
                .map_err(|err| err.to_string())?;
            Ok(CacheStatsView {
                stations: stats.stations,
                station_lines: stats.station_lines,
                line_detail: stats.line_detail,
                all_lines: stats.all_lines,
            })
        })
    }

    fn clear_cache(&self) -> Result<CacheStatsView, String> {
        self.runtime.block_on(async {
            self.services
                .clear_cache()
                .await
                .map_err(|err| err.to_string())?;
            let stats = self
                .services
                .cache_stats()
                .await
                .map_err(|err| err.to_string())?;
            Ok(CacheStatsView {
                stations: stats.stations,
                station_lines: stats.station_lines,
                line_detail: stats.line_detail,
                all_lines: stats.all_lines,
            })
        })
    }

    fn clear_cache_category(&self, index: i32) -> Result<CacheStatsView, String> {
        let category = match index {
            0 => CacheCategory::Stations,
            1 => CacheCategory::StationLines,
            2 => CacheCategory::LineDetail,
            _ => CacheCategory::AllLines,
        };
        self.runtime.block_on(async {
            self.services
                .clear_cache_category(category)
                .await
                .map_err(|err| err.to_string())?;
            let stats = self
                .services
                .cache_stats()
                .await
                .map_err(|err| err.to_string())?;
            Ok(CacheStatsView {
                stations: stats.stations,
                station_lines: stats.station_lines,
                line_detail: stats.line_detail,
                all_lines: stats.all_lines,
            })
        })
    }

    fn load_all_routes(&self) -> Result<Vec<BusRoute>, String> {
        self.runtime.block_on(async {
            self.services
                .all_routes()
                .await
                .map_err(|err| err.to_string())
        })
    }

    fn fetch_detail(
        &self,
        direction_id: &str,
        target_order: u32,
    ) -> Result<crate::domain::LineDetailSnapshot, String> {
        self.runtime.block_on(async {
            self
                .services
                .line_detail(direction_id, target_order)
                .await
                .map_err(|err| err.to_string())
        })
    }

    fn fetch_nearby(&self, lat: f64, lng: f64) -> (Result<NearbyData, String>, NetworkState) {
        self.runtime.block_on(async {
            let net_state = self.services.network_state().await;
            let result = self
                .services
                .nearby(lat, lng)
                .await
                .map(nearby_to_dto)
                .map_err(|err| err.to_string());
            (result, net_state)
        })
    }

    fn refresh_nearby(&self, lat: f64, lng: f64) -> (Result<NearbyData, String>, NetworkState) {
        self.runtime.block_on(async {
            let net_state = self.services.network_state().await;
            let result = self
                .services
                .refresh_nearby(lat, lng)
                .await
                .map(nearby_to_dto)
                .map_err(|err| err.to_string());
            (result, net_state)
        })
    }
}

impl SharedCoreBridge {
    fn shared() -> &'static Self {
        static BRIDGE: OnceLock<SharedCoreBridge> = OnceLock::new();
        BRIDGE.get_or_init(|| SharedCoreBridge {
            bridge: Mutex::new(CruxBridge::new(Core::new_with(
                WherebusApp::default(),
                Default::default(),
            ))),
        })
    }

    fn update(&self, event_json: &str) -> Result<String, String> {
        let mut bridge = self
            .bridge
            .lock()
            .map_err(|_| "crux bridge mutex poisoned".to_string())?;
        let mut requests = Vec::new();
        bridge
            .update(event_json.as_bytes(), &mut requests)
            .map_err(|err| err.to_string())?;
        self.drain_internal_requests(&mut bridge, requests)
    }

    fn resolve(&self, effect_id: u32, response_json: &str) -> Result<String, String> {
        let mut bridge = self
            .bridge
            .lock()
            .map_err(|_| "crux bridge mutex poisoned".to_string())?;
        let mut requests = Vec::new();
        bridge
            .resolve(EffectId(effect_id), response_json.as_bytes(), &mut requests)
            .map_err(|err| err.to_string())?;
        self.drain_internal_requests(&mut bridge, requests)
    }

    fn view(&self) -> Result<String, String> {
        let bridge = self
            .bridge
            .lock()
            .map_err(|_| "crux bridge mutex poisoned".to_string())?;
        let mut view = Vec::new();
        bridge
            .view(&mut view)
            .map_err(|err| err.to_string())?;
        String::from_utf8(view).map_err(|err| err.to_string())
    }

    fn drain_internal_requests(
        &self,
        bridge: &mut CruxBridge<WherebusApp, JsonFfiFormat>,
        requests: Vec<u8>,
    ) -> Result<String, String> {
        let mut queue = parse_requests(requests)?;
        let mut platform_requests = Vec::new();

        while let Some(request) = queue.pop_front() {
            match self.execute_internal_effect(&request.effect)? {
                InternalEffectResult::Resolve(response_json) => {
                    let mut produced = Vec::new();
                    bridge
                        .resolve(EffectId(request.id), response_json.as_bytes(), &mut produced)
                        .map_err(|err| err.to_string())?;
                    queue.extend(parse_requests(produced)?);
                }
                InternalEffectResult::NotifyOnly => {}
                InternalEffectResult::Platform => platform_requests.push(request),
            }
        }

        serde_json::to_string(&platform_requests).map_err(|err| err.to_string())
    }

    fn execute_internal_effect(&self, effect: &Value) -> Result<InternalEffectResult, String> {
        let Some(object) = effect.as_object() else {
            return Err("invalid effect payload".into());
        };
        let Some((kind, payload)) = object.iter().next() else {
            return Err("missing effect variant".into());
        };
        let engine = NativeEngine::shared()?;

        let response = match kind.as_str() {
            "RequestLocationPermission" | "FetchAutoLocation" => return Ok(InternalEffectResult::Platform),
            "Bootstrap" => serde_json::to_string(&engine.bootstrap_data()?).map(InternalEffectResult::Resolve),
            "SaveConfig" => {
                let op: SaveConfigOp =
                    serde_json::from_value(payload.clone()).map_err(|err| err.to_string())?;
                engine.save_config(op.config)?;
                Ok(InternalEffectResult::NotifyOnly)
            }
            "FetchNearby" => {
                let op: FetchNearbyOp =
                    serde_json::from_value(payload.clone()).map_err(|err| err.to_string())?;
                tracing::debug!("[bridge] FetchNearby lat={:.4} lng={:.4} force={}", op.lat, op.lng, op.force);
                let result = if op.force {
                    engine.refresh_nearby(op.lat, op.lng)
                } else {
                    engine.fetch_nearby(op.lat, op.lng)
                };
                serde_json::to_string(&result)
                    .map(InternalEffectResult::Resolve)
            }
            "LoadAllRoutes" => serde_json::to_string(&engine.load_all_routes())
                .map(InternalEffectResult::Resolve),
            "FetchDetail" => {
                let op: FetchDetailOp =
                    serde_json::from_value(payload.clone()).map_err(|err| err.to_string())?;
                serde_json::to_string(&engine.fetch_detail(&op.direction_id, op.target_order))
                    .map(InternalEffectResult::Resolve)
            }
            "SwitchService" => {
                let op: SwitchServiceOp =
                    serde_json::from_value(payload.clone()).map_err(|err| err.to_string())?;
                serde_json::to_string(&engine.switch_service(op.service_id))
                    .map(InternalEffectResult::Resolve)
            }
            "LoadCacheStats" => serde_json::to_string(&engine.cache_stats()?)
                .map(InternalEffectResult::Resolve),
            "ClearCache" => serde_json::to_string(&engine.clear_cache()?)
                .map(InternalEffectResult::Resolve),
            "ClearCacheCategory" => {
                let op: ClearCacheCategoryOp =
                    serde_json::from_value(payload.clone()).map_err(|err| err.to_string())?;
                serde_json::to_string(&engine.clear_cache_category(op.index)?)
                    .map(InternalEffectResult::Resolve)
            }
            other => return Err(format!("unsupported internal effect: {other}")),
        }
        .map_err(|err| err.to_string())?;

        Ok(response)
    }
}

impl BridgeEnvelope {
    fn success(data: String) -> Self {
        Self {
            ok: true,
            data: Some(data),
            error: None,
        }
    }

    fn failure(error: impl Into<String>) -> Self {
        Self {
            ok: false,
            data: None,
            error: Some(error.into()),
        }
    }
}

fn parse_requests(requests: Vec<u8>) -> Result<VecDeque<BridgeRequest>, String> {
    if requests.is_empty() {
        return Ok(VecDeque::new());
    }
    let json = String::from_utf8(requests).map_err(|err| err.to_string())?;
    if json.trim().is_empty() {
        return Ok(VecDeque::new());
    }
    let parsed: Vec<BridgeRequest> = serde_json::from_str(&json).map_err(|err| err.to_string())?;
    Ok(parsed.into())
}

fn bridge_update_json(event_json: &str) -> String {
    let envelope = match SharedCoreBridge::shared().update(event_json) {
        Ok(data) => BridgeEnvelope::success(data),
        Err(error) => BridgeEnvelope::failure(error),
    };
    serde_json::to_string(&envelope).unwrap_or_else(|_| {
        "{\"ok\":false,\"error\":\"bridge serialization failure\"}".to_string()
    })
}

fn bridge_resolve_json(effect_id: u32, response_json: &str) -> String {
    let envelope = match SharedCoreBridge::shared().resolve(effect_id, response_json) {
        Ok(data) => BridgeEnvelope::success(data),
        Err(error) => BridgeEnvelope::failure(error),
    };
    serde_json::to_string(&envelope).unwrap_or_else(|_| {
        "{\"ok\":false,\"error\":\"bridge serialization failure\"}".to_string()
    })
}

fn bridge_view_json() -> String {
    let envelope = match SharedCoreBridge::shared().view() {
        Ok(data) => BridgeEnvelope::success(data),
        Err(error) => BridgeEnvelope::failure(error),
    };
    serde_json::to_string(&envelope).unwrap_or_else(|_| {
        "{\"ok\":false,\"error\":\"bridge serialization failure\"}".to_string()
    })
}

fn build_city_picker_items(services: Vec<ServiceInfo>) -> Vec<CityItemView> {
    let mut items = Vec::new();
    let mut grouped: BTreeMap<String, Vec<ServiceInfo>> = BTreeMap::new();

    for service in services {
        grouped
            .entry(service.province.clone())
            .or_default()
            .push(service);
    }

    let mut region_order: HashMap<&'static str, usize> = HashMap::new();
    for (index, city) in ALL_CITIES.iter().enumerate() {
        region_order.entry(city.province()).or_insert(index);
    }

    let mut regions: Vec<_> = grouped.into_iter().collect();
    regions.sort_by_key(|(region, _)| {
        region_order
            .get(region.as_str())
            .copied()
            .unwrap_or(usize::MAX)
    });

    for (region, mut region_services) in regions {
        region_services.sort_by(|left, right| left.city_name.cmp(&right.city_name));
        items.push(CityItemView {
            name: region,
            provider: String::new(),
            service_id: String::new(),
            is_header: true,
        });

        for service in region_services {
            items.push(CityItemView {
                name: service.city_name,
                provider: service.provider_name,
                service_id: service.id,
                is_header: false,
            });
        }
    }

    items
}

fn nearby_to_dto(snapshot: crate::domain::NearbySnapshot) -> NearbyData {
    let mut items = Vec::new();
    let mut nearby_lines = Vec::new();

    for group in snapshot.groups {
        items.push(NearbyItemView {
            is_header: true,
            header: group.station.clone(),
            line: Default::default(),
            flat_index: items.len() as i32,
        });
        nearby_lines.push(None);

        for line in group.lines {
            let arrival = match line.arrival_state.as_str() {
                "Arriving" => ArrivalEstimate::Arriving,
                "Approaching" => ArrivalEstimate::Approaching {
                    stations_away: line.stations_away.max(0) as u32,
                    minutes_away: if line.minutes_away >= 0 {
                        Some(line.minutes_away as u32)
                    } else {
                        None
                    },
                    distance_m: if line.distance_m > 0 {
                        Some(line.distance_m as u32)
                    } else {
                        None
                    },
                },
                "NoService" => ArrivalEstimate::NoService,
                _ => ArrivalEstimate::Unknown,
            };
            let summary = LineSummary {
                id: None,
                name: line.line_name.clone(),
                direction_id: line.direction_id.clone(),
                endpoints: line.endpoints.clone(),
                arrival,
                station_order: 0,
                run_state: line.run_state.unwrap_or(RunState::NoRealtime),
            };

            items.push(NearbyItemView {
                is_header: false,
                header: Default::default(),
                line: line.clone(),
                flat_index: items.len() as i32,
            });
            nearby_lines.push(Some(summary));
        }
    }

    NearbyData {
        items,
        nearby_lines,
    }
}


use jni::EnvUnowned;
use jni::errors::ThrowRuntimeExAndDefault;
use jni::objects::{JClass, JString};
use jni::sys::jint;

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_noctiro_wherebus_data_NativeWhereBusBridge_setDataDir<'local>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
    path: JString<'local>,
) {
    use std::sync::Once;
    static INIT_TRACING: Once = Once::new();
    INIT_TRACING.call_once(|| {
        use tracing_subscriber::layer::SubscriberExt;
        use tracing_subscriber::util::SubscriberInitExt;
        if let Ok(layer) = tracing_android::layer("wherebus") {
            tracing_subscriber::registry()
                .with(tracing_subscriber::EnvFilter::new("wherebus=debug"))
                .with(layer)
                .init();
        }
    });

    unowned_env.with_env(|env| -> jni::errors::Result<()> {
        let path = path.try_to_string(env)?;
        if !path.is_empty() {
            set_android_data_dir(path);
        }
        Ok(())
    }).resolve::<ThrowRuntimeExAndDefault>();
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_noctiro_wherebus_data_NativeWhereBusBridge_update<'local>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
    event_json: JString<'local>,
) -> JString<'local> {
    unowned_env.with_env(|env| {
        let event_json = event_json.try_to_string(env)?;
        let response = bridge_update_json(&event_json);
        JString::from_str(env, &response)
    }).resolve::<ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_noctiro_wherebus_data_NativeWhereBusBridge_resolve<'local>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
    effect_id: jint,
    response_json: JString<'local>,
) -> JString<'local> {
    unowned_env.with_env(|env| {
        let response_json = response_json.try_to_string(env)?;
        let response = bridge_resolve_json(effect_id as u32, &response_json);
        JString::from_str(env, &response)
    }).resolve::<ThrowRuntimeExAndDefault>()
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_noctiro_wherebus_data_NativeWhereBusBridge_view<'local>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
) -> JString<'local> {
    unowned_env.with_env(|env| {
        let response = bridge_view_json();
        JString::from_str(env, &response)
    }).resolve::<ThrowRuntimeExAndDefault>()
}
