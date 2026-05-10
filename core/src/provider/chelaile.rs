pub mod cities;
pub mod client;
pub mod raw;

use crate::models::*;
use crate::providers::{BusDataProvider, ProviderError};
use async_trait::async_trait;
use client::ChelaileClient;
use raw::NearbyResponse;
use std::sync::Mutex;

pub struct ChelaileProvider {
    client: ChelaileClient,
    city_display: String,
    cached_nearby: Mutex<Option<NearbyResponse>>,
}

impl ChelaileProvider {
    pub fn new(city_id: &str, display_name: &str) -> Result<Self, ProviderError> {
        Ok(Self {
            client: ChelaileClient::new(city_id)?,
            city_display: display_name.to_string(),
            cached_nearby: Mutex::new(None),
        })
    }
}

fn infer_run_state_from_text(text: &str) -> RunState {
    let text = text.trim();
    if text.is_empty() {
        RunState::NoRealtime
    } else if text.contains("末班") || text.contains("等待发车") || text.contains("未发车") || text.contains("非运营") {
        RunState::NotOperating
    } else if text.contains("停运") || text.contains("收班") {
        RunState::Stopped
    } else {
        RunState::Running
    }
}

fn parse_chelaile_arrival(desc: &str, state: i32, distance_to_sp: u32) -> ArrivalEstimate {
    let text = desc.trim();

    if state == -3 || text.contains("末班时间已过") {
        return ArrivalEstimate::NoService;
    }
    if state == -1 || text.contains("等待发车") || text.contains("未发车") {
        return ArrivalEstimate::NoService;
    }
    if text.is_empty() {
        return ArrivalEstimate::Unknown;
    }
    if text.contains("即将到站") || text.contains("进站中") || text.contains("已到站") {
        return ArrivalEstimate::Arriving;
    }

    let stations_away = extract_stations_from_desc(text);
    let minutes_away = extract_minutes_from_desc(text);

    let distance_m = if distance_to_sp > 0 {
        Some(distance_to_sp)
    } else {
        None
    };

    if stations_away.is_some() || minutes_away.is_some() {
        let minutes = minutes_away.or_else(|| {
            distance_m.filter(|&d| d > 0).map(|d| (d / 400).max(1))
        });
        ArrivalEstimate::Approaching {
            stations_away: stations_away.unwrap_or(0),
            minutes_away: minutes,
            distance_m,
        }
    } else {
        ArrivalEstimate::Unknown
    }
}

fn extract_stations_from_desc(text: &str) -> Option<u32> {
    let idx = text.find('站')?;
    let before = &text[..idx];
    let num_str: String = before
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    num_str.parse().ok()
}

fn extract_minutes_from_desc(text: &str) -> Option<u32> {
    if text.contains("小于") || text.contains("不到") {
        return Some(0);
    }
    let idx = text.find('分')?;
    let before = &text[..idx];
    let num_str: String = before
        .chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    num_str.parse().ok()
}

fn infer_realtime_run_state(resp: &raw::LineDetailResponse) -> RunState {
    if resp.buses.is_empty() {
        return RunState::NoRealtime;
    }

    if resp
        .buses
        .iter()
        .all(|bus| bus.time_str.contains("未发车") || bus.time_str.contains("等待发车"))
    {
        return RunState::NotOperating;
    }

    if resp
        .buses
        .iter()
        .all(|bus| bus.time_str.contains("停运") || bus.time_str.contains("收班"))
    {
        return RunState::Stopped;
    }

    RunState::Running
}

#[async_trait]
impl BusDataProvider for ChelaileProvider {
    fn provider_name(&self) -> &str {
        "车来了"
    }
    fn city_name(&self) -> &str {
        &self.city_display
    }

    async fn nearby_stations(&self, lat: f64, lng: f64) -> Result<Vec<Station>, ProviderError> {
        let resp = self.client.nearby_lines(lat, lng).await?;
        let stations = resp
            .near_lines
            .iter()
            .map(|s| Station {
                id: Some(s.s_id.clone()),
                name: s.station_name.clone(),
                alias: None,
                lat: s.lat,
                lng: s.lng,
                distance_m: s.distance as u32,
                same_name_count: 0,
            })
            .collect();
        *self.cached_nearby.lock().unwrap() = Some(resp);
        Ok(stations)
    }

    async fn station_lines(
        &self,
        station: &str,
        lat: f64,
        lng: f64,
    ) -> Result<Vec<LineSummary>, ProviderError> {
        let resp = {
            let cached = self.cached_nearby.lock().unwrap();
            cached.clone()
        };

        let resp = match resp {
            Some(r) => r,
            None => self.client.nearby_lines(lat, lng).await?,
        };

        let target = resp
            .near_lines
            .into_iter()
            .find(|s| s.station_name == station);

        let lines = match target {
            Some(s) => s.lines,
            None => return Ok(Vec::new()),
        };

        Ok(lines
            .into_iter()
            .map(|lw| {
                let target_order = lw.target_station.as_ref().map(|t| t.order).unwrap_or(0);
                let distance_to_sp = lw.target_station.as_ref().map(|t| t.distance_to_sp).unwrap_or(0);

                let desc = lw.line.desc.clone();
                let state = lw.line.state;
                let run_state = infer_run_state_from_text(&desc);
                let arrival = parse_chelaile_arrival(&desc, state, distance_to_sp);
                tracing::debug!(
                    "[chelaile] {} desc=\"{}\" state={} dist={} → {:?}",
                    lw.line.name, desc, state, distance_to_sp, arrival
                );

                LineSummary {
                    id: Some(lw.line.line_id.clone()),
                    name: lw.line.name,
                    direction_id: lw.line.line_id,
                    endpoints: EndpointsView {
                        origin: lw.line.start_sn,
                        origin_alias: None,
                        terminus: lw.line.end_sn,
                        terminus_alias: None,
                    },
                    arrival,
                    station_order: target_order,
                    run_state,
                }
            })
            .collect())
    }

    async fn line_detail(
        &self,
        key: &str,
    ) -> Result<LineDetail, ProviderError> {
        let resp = self.client.line_route(key).await?;

        let start_stop = resp
            .stations
            .first()
            .map(|s| s.station_name.clone())
            .unwrap_or_default();
        let end_stop = resp
            .stations
            .last()
            .map(|s| s.station_name.clone())
            .unwrap_or_default();

        let line_name = resp
            .line
            .as_ref()
            .map(|l| l.line_no.clone())
            .unwrap_or_default();

        let (first_time, last_time, price, company) = if let Some(ref line) = resp.line {
            (
                ServiceTime::parse(&line.first_time),
                ServiceTime::parse(&line.last_time),
                Some(line.price.clone()).filter(|s| !s.is_empty()),
                Some(line.company.clone()).filter(|s| !s.is_empty()),
            )
        } else {
            (None, None, None, None)
        };

        Ok(LineDetail {
            id: Some(key.to_string()),
            name: line_name,
            direction_id: key.to_string(),
            reverse_id: Self::make_reverse_id(key),
            topology: RouteTopology {
                start: Terminal::named(start_stop),
                end: Terminal::named(end_stop),
                stations: resp
                    .stations
                    .into_iter()
                    .map(|s| LineStop {
                        id: Some(s.s_id),
                        name: s.station_name,
                        alias: None,
                        order: s.order,
                        lat: s.lat,
                        lng: s.lng,
                        status: StopStatus::Normal,
                        track_index: None,
                    })
                    .collect(),
                track_points: Vec::new(),
            },
            meta: LineMeta {
                first_service: first_time,
                last_service: last_time,
                fare: price.map(Fare::text).unwrap_or(Fare::Unknown),
                company,
                contact: ContactInfo::Unknown,
                notes: None,
            },
        })
    }

    async fn realtime(
        &self,
        key: &str,
        target_order: u32,
    ) -> Result<RealTimeData, ProviderError> {
        let route = self.client.line_route(key).await?;
        let line_name = route
            .line
            .as_ref()
            .map(|l| l.line_no.clone())
            .unwrap_or_default();
        let direction_str = Self::extract_direction_from_key(key);
        let order_idx = target_order.saturating_sub(1) as usize;
        let station_name = route
            .stations
            .get(order_idx)
            .map(|s| s.station_name.as_str())
            .unwrap_or("");
        let next_station_name = route
            .stations
            .get(order_idx + 1)
            .map(|s| s.station_name.as_str())
            .unwrap_or("");

        let resp = self
            .client
            .line_detail(
                key,
                &line_name,
                &direction_str,
                station_name,
                next_station_name,
                &line_name,
                target_order,
            )
            .await?;

        let run_state = infer_realtime_run_state(&resp);
        tracing::debug!(
            "[chelaile] realtime {} order={} buses={}",
            line_name, target_order, resp.buses.len()
        );

        let buses: Vec<BusPosition> = resp
            .buses
            .into_iter()
            .map(|b| {
                let travel_secs = if b.travel_time > 0.0 {
                    Some(b.travel_time as u32)
                } else {
                    None
                };
                BusPosition {
                    bus_id: b.bus_id,
                    station_index: b.order,
                    is_arriving: b.state == "0",
                    lat: b.lat,
                    lng: b.lng,
                    angle: None,
                    distance_to_station: Some(b.distance_to_sc),
                    travel_time_secs: travel_secs,
                    station_name: None,
                    crowd_status: CrowdLevel::Unknown,
                    track_segment_index: None,
                    state_description: Some(b.time_str).filter(|s| !s.is_empty()),
                }
            })
            .collect();

        let arrival_estimates: Vec<ArrivalDetail> = buses
            .iter()
            .filter(|b| b.station_index < target_order && b.travel_time_secs.is_some())
            .map(|b| ArrivalDetail {
                stations_away: target_order.saturating_sub(b.station_index),
                minutes_away: b.travel_time_secs.map(|s| (s / 60).max(1)).unwrap_or(0),
                distance_m: b.distance_to_station.unwrap_or(0.0) as u32,
            })
            .collect();

        Ok(RealTimeData {
            run_state,
            plan_time: None,
            buses,
            station_arrivals: Vec::new(),
            segments: Vec::new(),
            arrival_estimates,
        })
    }

    async fn all_lines(&self) -> Result<Vec<BusRoute>, ProviderError> {
        let resp = self.client.search_lines("").await?;
        Ok(resp
            .all_lines
            .all
            .into_iter()
            .map(|l| {
                let endpoints = EndpointsView {
                    origin: l.start_stop_name.clone(),
                    origin_alias: None,
                    terminus: l.end_stop_name.clone(),
                    terminus_alias: None,
                };
                BusRoute {
                    id: Some(l.line_id.clone()),
                    name: l.line_name,
                    direction_id: l.line_id,
                    terminals: (
                        Terminal::named(l.start_stop_name),
                        Terminal::named(l.end_stop_name),
                    ),
                    endpoints,
                    company: None,
                }
            })
            .collect())
    }
}

impl ChelaileProvider {
    fn extract_direction_from_key(key: &str) -> String {
        if let Some(pos) = key.rfind('-') {
            let suffix = &key[pos + 1..];
            if suffix.len() <= 2 && suffix.chars().all(|c| c.is_ascii_digit()) {
                return suffix.to_string();
            }
        }
        String::new()
    }

    fn make_reverse_id(key: &str) -> Option<String> {
        let pos = key.rfind('-')?;
        let suffix = &key[pos + 1..];
        match suffix {
            "0" => Some(format!("{}1", &key[..=pos])),
            "1" => Some(format!("{}0", &key[..=pos])),
            _ => None,
        }
    }
}
