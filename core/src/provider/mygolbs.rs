pub mod cities;
pub mod client;
pub mod raw;

use crate::models::*;
use crate::providers::{BusDataProvider, ProviderError};
use async_trait::async_trait;
use client::GolbsClient;
use raw::CityConfig;

pub struct MygolbsProvider {
    client: GolbsClient,
    city_display: String,
}

impl MygolbsProvider {
    pub fn new(city_name: &str, city_key: &str, display_name: &str) -> Result<Self, ProviderError> {
        let config = CityConfig {
            name: city_name.to_string(),
            key: city_key.to_string(),
        };
        Ok(Self {
            client: GolbsClient::new(config)?,
            city_display: display_name.to_string(),
        })
    }

    pub fn from_city(city: &cities::MygolbsCity) -> Result<Self, ProviderError> {
        Self::new(city.api_city_name, city.api_city_key, city.city.name())
    }

    fn parse_direction_id(key: &str) -> (String, String) {
        match key.rsplit_once(':') {
            Some((name, dir)) => (name.to_string(), dir.to_string()),
            None => (key.to_string(), "1".to_string()),
        }
    }
}

fn infer_run_state_from_text(text: &str) -> RunState {
    let text = text.trim();
    if text.is_empty() {
        RunState::NoRealtime
    } else if text.contains("非运营") || text.contains("未发车") || text.contains("等待发车")
    {
        RunState::NotOperating
    } else if text.contains("停运") || text.contains("收班") {
        RunState::Stopped
    } else {
        RunState::Running
    }
}

fn infer_realtime_run_state(resp: &raw::RealTimeResponse) -> RunState {
    match (resp.run_state, resp.has_real) {
        (1, _) => RunState::Stopped,
        (_, 0) => {
            if resp
                .plan_time
                .as_deref()
                .is_some_and(|time| !time.trim().is_empty())
            {
                RunState::NotOperating
            } else {
                RunState::NoRealtime
            }
        }
        _ => RunState::Running,
    }
}

fn parse_arrival(neartext: &str, neardis: &str, neartime: &str, nearnum: i32) -> ArrivalEstimate {
    let text = neartext.trim();
    if text.is_empty() {
        return ArrivalEstimate::Unknown;
    }
    if text.contains("即将到站") || text.contains("进站中") || text.contains("已到站") {
        return ArrivalEstimate::Arriving;
    }
    if text.contains("等待发车") || nearnum < 0 {
        return ArrivalEstimate::NoService;
    }

    let stations_away = if nearnum > 0 {
        Some(nearnum as u32)
    } else {
        extract_stations(text)
    };

    let (neardis_minutes, distance_m) = parse_neardis(neardis);

    let minutes_away = parse_time_field(neartime)
        .or_else(|| extract_minutes(text))
        .or(neardis_minutes);

    if stations_away.is_some() || minutes_away.is_some() {
        let minutes = minutes_away.or_else(|| estimate_minutes(distance_m));
        ArrivalEstimate::Approaching {
            stations_away: stations_away.unwrap_or(0),
            minutes_away: minutes,
            distance_m,
        }
    } else {
        ArrivalEstimate::Unknown
    }
}

fn parse_time_field(neartime: &str) -> Option<u32> {
    let s = neartime.trim();
    if s.is_empty() {
        return None;
    }
    if s.contains("小于") || s.contains("不到") {
        return Some(0);
    }
    let num_str: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    num_str.parse().ok()
}

const BUS_SPEED_M_PER_MIN: u32 = 400;

fn estimate_minutes(distance_m: Option<u32>) -> Option<u32> {
    let d = distance_m.filter(|&d| d > 0)?;
    Some((d / BUS_SPEED_M_PER_MIN).max(1))
}

fn extract_stations(text: &str) -> Option<u32> {
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

fn extract_minutes(text: &str) -> Option<u32> {
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

/// Parse neardis field which can be:
/// - "19分钟/6.2公里" (minutes + distance)
/// - "780米" or "1.2km" (distance only)
/// - "" (empty)
/// Returns (minutes, distance_m)
fn parse_neardis(text: &str) -> (Option<u32>, Option<u32>) {
    let text = text.trim();
    if text.is_empty() {
        return (None, None);
    }
    if let Some(slash) = text.find('/') {
        let left = &text[..slash];
        let right = &text[slash + 1..];
        let minutes = extract_minutes_from_part(left);
        let distance = parse_distance_part(right);
        return (minutes, distance);
    }
    if text.contains('分') {
        return (extract_minutes_from_part(text), None);
    }
    (None, parse_distance_part(text))
}

fn extract_minutes_from_part(s: &str) -> Option<u32> {
    let num_str: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    let val: u32 = num_str.parse().ok()?;
    if val > 0 { Some(val) } else { None }
}

fn parse_distance_part(text: &str) -> Option<u32> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    if let Some(km) = text
        .strip_suffix("km")
        .or_else(|| text.strip_suffix("公里"))
    {
        let val: f64 = km.trim().parse().ok()?;
        return Some((val * 1000.0) as u32);
    }
    if let Some(m) = text.strip_suffix('m').or_else(|| text.strip_suffix("米")) {
        let val: f64 = m.trim().parse().ok()?;
        return Some(val as u32);
    }
    text.parse::<f64>().ok().map(|v| v as u32)
}

fn parse_station_name(raw: &str) -> (String, Option<String>) {
    let raw = raw.trim();
    for (open, close) in [('︵', '︶'), ('（', '）'), ('(', ')')] {
        if let Some(pos) = raw.find(open) {
            let name = raw[..pos].trim().to_string();
            let rest = &raw[pos + open.len_utf8()..];
            let alias = rest.trim_end_matches(close).trim().to_string();
            if !alias.is_empty() {
                return (name, Some(alias));
            }
            return (name, None);
        }
    }
    (raw.to_string(), None)
}

fn congestion_from_color(color: &str) -> CongestionLevel {
    match color {
        "green" | "#00ff00" | "#4caf50" => CongestionLevel::Smooth,
        "yellow" | "orange" | "#ff9800" | "#ffeb3b" => CongestionLevel::Slow,
        "red" | "#f44336" | "#ff0000" => CongestionLevel::Congested,
        _ => CongestionLevel::Unknown,
    }
}

#[async_trait]
impl BusDataProvider for MygolbsProvider {
    fn provider_name(&self) -> &str {
        "掌上公交"
    }
    fn city_name(&self) -> &str {
        &self.city_display
    }

    async fn nearby_stations(&self, lat: f64, lng: f64) -> Result<Vec<Station>, ProviderError> {
        let resp = self.client.nearby_stations(lat, lng).await?;
        Ok(resp
            .data
            .into_iter()
            .map(|s| {
                let (name, alias) = parse_station_name(&s.name);
                Station {
                    id: None,
                    name,
                    alias,
                    lat: s.lat.parse().unwrap_or(0.0),
                    lng: s.lon.parse().unwrap_or(0.0),
                    distance_m: s.dis,
                    same_name_count: s.same_num,
                }
            })
            .collect())
    }

    async fn station_lines(
        &self,
        station: &str,
        lat: f64,
        lng: f64,
    ) -> Result<Vec<LineSummary>, ProviderError> {
        let lat_s = lat.to_string();
        let lng_s = lng.to_string();
        let resp = self
            .client
            .station_lines(station, &lat_s, &lng_s, false)
            .await?;
        Ok(resp
            .data
            .into_iter()
            .map(|l| {
                let run_state = infer_run_state_from_text(&l.neartext);
                let arrival = parse_arrival(&l.neartext, &l.neardis, &l.neartime, l.nearnum);
                tracing::debug!(
                    "[mygolbs] {} neartext=\"{}\" neartime=\"{}\" nearnum={} neardis=\"{}\" dir=\"{}\" → {:?}",
                    l.line_name, l.neartext, l.neartime, l.nearnum, l.neardis, l.direction, arrival
                );
                let direction_code = if l.direction.is_empty() {
                    "1".to_string()
                } else {
                    l.direction.clone()
                };
                let direction_id = format!("{}:{}", l.line_name, direction_code);
                LineSummary {
                    id: None,
                    name: l.line_name,
                    direction_id,
                    endpoints: EndpointsView::default(),
                    arrival,
                    station_order: l.station_order,
                    run_state,
                }
            })
            .collect())
    }

    async fn line_detail(&self, key: &str) -> Result<LineDetail, ProviderError> {
        let (name, direction_raw) = Self::parse_direction_id(key);
        let resp = self.client.line_stations(&name, &direction_raw).await?;
        let first_time = resp
            .first_last
            .first()
            .and_then(|fl| ServiceTime::parse(&fl.first));
        let last_time = resp
            .first_last
            .first()
            .and_then(|fl| ServiceTime::parse(&fl.last));
        let start_stop = resp
            .data
            .first()
            .map(|s| {
                let (n, _) = parse_station_name(&s.show_name);
                n
            })
            .unwrap_or_default();
        let end_stop = resp
            .data
            .last()
            .map(|s| {
                let (n, _) = parse_station_name(&s.show_name);
                n
            })
            .unwrap_or_default();

        let actual_key = format!("{}:{}", resp.route_name, resp.direction);
        let reverse_id = {
            let flipped = match resp.direction.as_str() {
                "1" => "2",
                "2" => "1",
                _ => "",
            };
            if flipped.is_empty() {
                None
            } else {
                Some(format!("{}:{}", resp.route_name, flipped))
            }
        };

        Ok(LineDetail {
            id: None,
            name: resp.route_name,
            direction_id: actual_key,
            reverse_id,
            topology: RouteTopology {
                start: Terminal::named(start_stop),
                end: Terminal::named(end_stop),
                stations: resp
                    .data
                    .into_iter()
                    .map(|s| {
                        let (name, alias) = parse_station_name(&s.show_name);
                        LineStop {
                            id: None,
                            name,
                            alias,
                            order: s.station_order,
                            lat: s.lat.parse().unwrap_or(0.0),
                            lng: s.lng.parse().unwrap_or(0.0),
                            status: StopStatus::from(s.status),
                            track_index: Some(s.nihe_point_index),
                        }
                    })
                    .collect(),
                track_points: resp
                    .nihelist
                    .into_iter()
                    .filter_map(|p| Some((p.lat.parse::<f64>().ok()?, p.lng.parse::<f64>().ok()?)))
                    .collect(),
            },
            meta: LineMeta {
                first_service: first_time,
                last_service: last_time,
                fare: Fare::Unknown,
                company: None,
                contact: ContactInfo::Unknown,
                notes: Some(resp.comments).filter(|value| !value.trim().is_empty()),
            },
        })
    }

    async fn realtime(&self, key: &str, order: u32) -> Result<RealTimeData, ProviderError> {
        let (name, direction_raw) = Self::parse_direction_id(key);
        let resp = self.client.realtime(&name, &direction_raw, order).await?;
        let run_state = infer_realtime_run_state(&resp);
        tracing::debug!(
            "[mygolbs] realtime {} order={} runState={} hasReal={} buses={} rtime={}",
            name,
            order,
            resp.run_state,
            resp.has_real,
            resp.list.len(),
            resp.rtime_list.len()
        );

        // speed 单位为 m/min，转换为 km/h: speed * 60 / 1000
        let segments: Vec<RouteSegment> = resp
            .dislist
            .iter()
            .zip(resp.speedlist.iter())
            .map(|(d, s)| RouteSegment {
                distance: Some(d.d),
                speed: if s.speed > 0.0 {
                    Some(s.speed * 60.0 / 1000.0)
                } else {
                    None
                },
                congestion: congestion_from_color(&s.co),
            })
            .collect();

        Ok(RealTimeData {
            run_state,
            plan_time: resp.plan_time,
            buses: resp
                .list
                .into_iter()
                .map(|b| BusPosition {
                    bus_id: b.bus_number,
                    station_index: b.index,
                    is_arriving: b.status_type == "0",
                    lat: b.lat.parse().ok(),
                    lng: b.lng.parse().ok(),
                    angle: Some(b.angle),
                    distance_to_station: Some(b.bus_to_station_distance),
                    travel_time_secs: None,
                    station_name: if b.station_name.is_empty() {
                        None
                    } else {
                        Some(b.station_name)
                    },
                    crowd_status: CrowdLevel::Unknown,
                    track_segment_index: Some(b.nihe_point_index),
                    state_description: None,
                })
                .collect(),
            station_arrivals: resp
                .data
                .into_iter()
                .map(|s| StationArrival {
                    station_index: s.index,
                    station_name: if s.station_name.is_empty() {
                        None
                    } else {
                        Some(s.station_name)
                    },
                    arriving_count: s.arrive,
                    leaving_count: s.come,
                })
                .collect(),
            segments,
            arrival_estimates: resp
                .rtime_list
                .into_iter()
                .filter(|r| r.count >= 0)
                .map(|r| ArrivalDetail {
                    stations_away: r.count.max(0) as u32,
                    minutes_away: r.time.ceil() as u32,
                    distance_m: r.distance,
                })
                .collect(),
        })
    }

    async fn all_lines(&self) -> Result<Vec<BusRoute>, ProviderError> {
        let resp = self.client.all_lines().await?;
        Ok(resp
            .buslines
            .into_iter()
            .map(|l| {
                let direction_id = format!("{}:{}", l.line_name, l.direction);
                let (origin, origin_alias) = parse_station_name(&l.from);
                let (terminus, terminus_alias) = parse_station_name(&l.to);
                BusRoute {
                    id: None,
                    name: l.line_name,
                    direction_id,
                    terminals: (Terminal::named(&l.from), Terminal::named(&l.to)),
                    endpoints: EndpointsView {
                        origin,
                        origin_alias,
                        terminus,
                        terminus_alias,
                    },
                    company: if l.company.is_empty() {
                        None
                    } else {
                        Some(l.company)
                    },
                }
            })
            .collect())
    }
}
