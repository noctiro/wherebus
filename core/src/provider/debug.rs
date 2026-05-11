use async_trait::async_trait;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::domain::transit::*;
use crate::provider::{BusDataProvider, ProviderError};

pub const SERVICE_ID: &str = "debug_beijing";

struct Rng(u64);

impl Rng {
    fn from_time() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        Self(seed)
    }

    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn range(&mut self, min: u64, max: u64) -> u64 {
        if min >= max { return min; }
        min + self.next() % (max - min + 1)
    }

    fn float(&mut self) -> f64 {
        (self.next() % 10000) as f64 / 10000.0
    }

    fn pick<T: Copy>(&mut self, items: &[T]) -> T {
        items[(self.next() as usize) % items.len()]
    }
}

fn time_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

struct SimRoute {
    id: &'static str,
    name: &'static str,
    dir_id: &'static str,
    reverse_id: &'static str,
    origin: &'static str,
    origin_alias: Option<&'static str>,
    terminus: &'static str,
    terminus_alias: Option<&'static str>,
    company: &'static str,
    first_service: &'static str,
    last_service: &'static str,
    fare: &'static str,
    contact: &'static str,
    notes: Option<&'static str>,
    stops: &'static [SimStop],
}

struct SimStop {
    id: &'static str,
    name: &'static str,
    alias: Option<&'static str>,
    lat: f64,
    lng: f64,
    status: StopStatus,
}

// 手动定位推荐坐标（nearby_stations 搜索半径 2km）：
// - 天安门附近: 39.9087, 116.4074（命中1路、夜1路）
// - 西直门附近: 39.9420, 116.3530（命中52路）
// - 中关村附近: 39.9800, 116.3100（命中特8路）
// - 公主坟附近: 39.9072, 116.3120（命中1路、夜1路、特8路）
// - 望京西附近: 39.9900, 116.4680（命中快速直达专线1）
// - 全覆盖中心: 39.9100, 116.3800（命中大部分线路）
const ROUTES: &[SimRoute] = &[
    SimRoute {
        id: "dbg_line_1", name: "1路", dir_id: "dbg_dir_1", reverse_id: "dbg_dir_2",
        origin: "老山公交场站", origin_alias: Some("老山"),
        terminus: "四惠枢纽站", terminus_alias: Some("四惠东"),
        company: "北京公交集团第四客运分公司",
        first_service: "05:00", last_service: "23:00",
        fare: "2元", contact: "010-96166",
        notes: Some("冬季末班提前至22:30"),
        stops: &ROUTE1_STOPS,
    },
    SimRoute {
        id: "dbg_line_2", name: "52路", dir_id: "dbg_dir_3", reverse_id: "dbg_dir_4",
        origin: "平乐园", origin_alias: None,
        terminus: "西直门", terminus_alias: Some("西直门外"),
        company: "北京公交集团第三客运分公司",
        first_service: "05:30", last_service: "22:00",
        fare: "2元", contact: "010-96166",
        notes: None,
        stops: &ROUTE2_STOPS,
    },
    SimRoute {
        id: "dbg_line_3", name: "特8路", dir_id: "dbg_dir_5", reverse_id: "dbg_dir_6",
        origin: "颐和园", origin_alias: Some("颐和园北宫门"),
        terminus: "前门", terminus_alias: Some("前门西"),
        company: "北京公交集团第一客运分公司",
        first_service: "06:00", last_service: "21:30",
        fare: "3元起步", contact: "010-96166",
        notes: Some("分段计价，全程5元"),
        stops: &ROUTE3_STOPS,
    },
    SimRoute {
        id: "dbg_line_4", name: "夜1路", dir_id: "dbg_dir_7", reverse_id: "dbg_dir_8",
        origin: "老山公交场站", origin_alias: Some("老山"),
        terminus: "四惠枢纽站", terminus_alias: Some("四惠东"),
        company: "北京公交集团第四客运分公司",
        first_service: "23:20", last_service: "04:50",
        fare: "2元", contact: "010-96166",
        notes: Some("夜间线路，与1路走向相同"),
        stops: &ROUTE4_STOPS,
    },
    SimRoute {
        id: "dbg_line_5", name: "快速直达专线1", dir_id: "dbg_dir_9", reverse_id: "dbg_dir_10",
        origin: "望京西", origin_alias: Some("望京西交通枢纽"),
        terminus: "北京西站", terminus_alias: Some("北京西站南广场"),
        company: "北京公交集团快速公交分公司",
        first_service: "06:30", last_service: "20:00",
        fare: "5元", contact: "010-96166",
        notes: Some("中途不停站，直达"),
        stops: &ROUTE5_STOPS,
    },
];

const ROUTE1_STOPS: [SimStop; 10] = [
    SimStop { id: "s1_01", name: "老山公交场站", alias: Some("老山"), lat: 39.9042, lng: 116.2365, status: StopStatus::Normal },
    SimStop { id: "s1_02", name: "八角游乐园", alias: None, lat: 39.9058, lng: 116.2580, status: StopStatus::Normal },
    SimStop { id: "s1_03", name: "玉泉路", alias: Some("玉泉路口西"), lat: 39.9065, lng: 116.2830, status: StopStatus::Normal },
    SimStop { id: "s1_04", name: "公主坟", alias: Some("公主坟北"), lat: 39.9072, lng: 116.3120, status: StopStatus::Normal },
    SimStop { id: "s1_05", name: "军事博物馆", alias: None, lat: 39.9078, lng: 116.3380, status: StopStatus::BoardOnly },
    SimStop { id: "s1_06", name: "木樨地", alias: None, lat: 39.9080, lng: 116.3560, status: StopStatus::Normal },
    SimStop { id: "s1_07", name: "天安门西", alias: None, lat: 39.9085, lng: 116.3910, status: StopStatus::Normal },
    SimStop { id: "s1_08", name: "天安门东", alias: Some("天安门东站"), lat: 39.9087, lng: 116.4074, status: StopStatus::Normal },
    SimStop { id: "s1_09", name: "王府井", alias: Some("王府井百货"), lat: 39.9133, lng: 116.4107, status: StopStatus::AlightOnly },
    SimStop { id: "s1_10", name: "四惠枢纽站", alias: Some("四惠东"), lat: 39.9082, lng: 116.4928, status: StopStatus::Normal },
];

const ROUTE2_STOPS: [SimStop; 9] = [
    SimStop { id: "s2_01", name: "平乐园", alias: None, lat: 39.8750, lng: 116.4820, status: StopStatus::Normal },
    SimStop { id: "s2_02", name: "劲松", alias: Some("劲松桥东"), lat: 39.8820, lng: 116.4530, status: StopStatus::Normal },
    SimStop { id: "s2_03", name: "双井", alias: None, lat: 39.8990, lng: 116.4580, status: StopStatus::Normal },
    SimStop { id: "s2_04", name: "国贸", alias: Some("国贸桥"), lat: 39.9080, lng: 116.4600, status: StopStatus::Temporary },
    SimStop { id: "s2_05", name: "东单", alias: Some("东单路口北"), lat: 39.9148, lng: 116.4162, status: StopStatus::OnDemand },
    SimStop { id: "s2_06", name: "灯市口", alias: None, lat: 39.9180, lng: 116.4100, status: StopStatus::Normal },
    SimStop { id: "s2_07", name: "西四", alias: None, lat: 39.9260, lng: 116.3720, status: StopStatus::Normal },
    SimStop { id: "s2_08", name: "新街口", alias: Some("新街口豁口"), lat: 39.9350, lng: 116.3620, status: StopStatus::Normal },
    SimStop { id: "s2_09", name: "西直门", alias: Some("西直门外"), lat: 39.9420, lng: 116.3530, status: StopStatus::Normal },
];

const ROUTE3_STOPS: [SimStop; 8] = [
    SimStop { id: "s3_01", name: "颐和园", alias: Some("颐和园北宫门"), lat: 40.0000, lng: 116.2750, status: StopStatus::Normal },
    SimStop { id: "s3_02", name: "北京大学西门", alias: Some("北大西门"), lat: 39.9920, lng: 116.2980, status: StopStatus::Normal },
    SimStop { id: "s3_03", name: "中关村", alias: Some("中关村南"), lat: 39.9800, lng: 116.3100, status: StopStatus::Normal },
    SimStop { id: "s3_04", name: "白石桥", alias: None, lat: 39.9550, lng: 116.3250, status: StopStatus::Express },
    SimStop { id: "s3_05", name: "甘家口", alias: None, lat: 39.9300, lng: 116.3350, status: StopStatus::Express },
    SimStop { id: "s3_06", name: "复兴门", alias: None, lat: 39.9080, lng: 116.3560, status: StopStatus::Normal },
    SimStop { id: "s3_07", name: "西单", alias: Some("西单商场"), lat: 39.9080, lng: 116.3730, status: StopStatus::Normal },
    SimStop { id: "s3_08", name: "前门", alias: Some("前门西"), lat: 39.8990, lng: 116.3970, status: StopStatus::Normal },
];

const ROUTE4_STOPS: [SimStop; 6] = [
    SimStop { id: "s4_01", name: "老山公交场站", alias: Some("老山"), lat: 39.9042, lng: 116.2365, status: StopStatus::Normal },
    SimStop { id: "s4_02", name: "公主坟", alias: Some("公主坟北"), lat: 39.9072, lng: 116.3120, status: StopStatus::Normal },
    SimStop { id: "s4_03", name: "军事博物馆", alias: None, lat: 39.9078, lng: 116.3380, status: StopStatus::Normal },
    SimStop { id: "s4_04", name: "天安门西", alias: None, lat: 39.9085, lng: 116.3910, status: StopStatus::NotStopping },
    SimStop { id: "s4_05", name: "天安门东", alias: Some("天安门东站"), lat: 39.9087, lng: 116.4074, status: StopStatus::NotStopping },
    SimStop { id: "s4_06", name: "四惠枢纽站", alias: Some("四惠东"), lat: 39.9082, lng: 116.4928, status: StopStatus::Normal },
];

const ROUTE5_STOPS: [SimStop; 4] = [
    SimStop { id: "s5_01", name: "望京西", alias: Some("望京西交通枢纽"), lat: 39.9900, lng: 116.4680, status: StopStatus::Normal },
    SimStop { id: "s5_02", name: "大望路", alias: None, lat: 39.9085, lng: 116.4620, status: StopStatus::Express },
    SimStop { id: "s5_03", name: "北京西站南广场", alias: Some("西站南"), lat: 39.8950, lng: 116.3220, status: StopStatus::Express },
    SimStop { id: "s5_04", name: "北京西站", alias: Some("北京西站南广场"), lat: 39.8960, lng: 116.3200, status: StopStatus::Normal },
];

const BUS_PLATES: &[&str] = &[
    "京A00001", "京A00002", "京A00003", "京A12345", "京B10086",
    "京B10087", "京B20088", "京C20001", "京C20002", "京C30003",
    "京D30001", "京D30002", "京D40004", "京E40001", "京E50005",
];

const CROWD_LEVELS: &[CrowdLevel] = &[
    CrowdLevel::Unknown, CrowdLevel::Spacious, CrowdLevel::Spacious,
    CrowdLevel::Normal, CrowdLevel::Normal, CrowdLevel::Normal,
    CrowdLevel::Crowded, CrowdLevel::Full,
];

const CONGESTION_LEVELS: &[CongestionLevel] = &[
    CongestionLevel::Smooth, CongestionLevel::Smooth, CongestionLevel::Smooth,
    CongestionLevel::Slow, CongestionLevel::Slow,
    CongestionLevel::Congested, CongestionLevel::Unknown,
];

pub struct DebugProvider;

impl DebugProvider {
    fn find_route(key: &str) -> &'static SimRoute {
        ROUTES.iter()
            .find(|r| r.id == key || r.dir_id == key || r.reverse_id == key)
            .unwrap_or(&ROUTES[0])
    }

    fn sim_run_state(route: &SimRoute, rng: &mut Rng) -> RunState {
        let t = time_secs();
        let hour = ((t / 3600) % 24) as u32;
        let first_h: u32 = route.first_service.split(':').next().unwrap_or("5").parse().unwrap_or(5);
        let last_h: u32 = route.last_service.split(':').next().unwrap_or("23").parse().unwrap_or(23);

        if first_h < last_h {
            if hour < first_h { return RunState::NotOperating; }
            if hour >= last_h { return RunState::Stopped; }
        }
        if rng.range(0, 20) == 0 { return RunState::NoRealtime; }
        RunState::Running
    }

    fn sim_buses(route: &SimRoute, rng: &mut Rng) -> Vec<BusPosition> {
        let n_stops = route.stops.len();
        let bus_count = rng.range(2, 5) as usize;
        let t = time_secs();
        let mut buses = Vec::with_capacity(bus_count);
        let mut used_plates = Vec::new();

        for _ in 0..bus_count {
            let mut plate_idx = (rng.next() as usize) % BUS_PLATES.len();
            while used_plates.contains(&plate_idx) {
                plate_idx = (plate_idx + 1) % BUS_PLATES.len();
            }
            used_plates.push(plate_idx);

            let cycle_period = (n_stops as u64) * 120;
            let bus_offset = rng.range(0, cycle_period);
            let phase = ((t + bus_offset) % cycle_period) as f64 / cycle_period as f64;
            let float_idx = phase * (n_stops - 1) as f64;
            let station_idx = (float_idx as usize).min(n_stops - 2);
            let progress = float_idx - station_idx as f64;

            let is_arriving = progress > 0.8;

            let stop = &route.stops[station_idx + 1];
            let prev = &route.stops[station_idx];
            let lat = prev.lat + (stop.lat - prev.lat) * progress;
            let lng = prev.lng + (stop.lng - prev.lng) * progress;

            let jitter_lat = (rng.float() - 0.5) * 0.0003;
            let jitter_lng = (rng.float() - 0.5) * 0.0003;

            let dist = ((stop.lat - lat).powi(2) + (stop.lng - lng).powi(2)).sqrt() * 111_000.0;
            let speed_mps = 6.0 + rng.float() * 10.0;
            let travel_secs = if dist < 1.0 { 0 } else { (dist / speed_mps) as u32 };

            let dy = stop.lat - prev.lat;
            let dx = stop.lng - prev.lng;
            let angle = dx.atan2(dy).to_degrees();
            let angle_jitter = (rng.float() - 0.5) * 10.0;

            let crowd = rng.pick(CROWD_LEVELS);

            let desc = if is_arriving {
                format!("{}即将到站", stop.name)
            } else if travel_secs < 60 {
                "即将到站".to_string()
            } else {
                format!("距{}还有{}分钟", stop.name, travel_secs / 60 + 1)
            };

            buses.push(BusPosition {
                bus_id: BUS_PLATES[plate_idx].into(),
                station_index: (station_idx + 1) as u32,
                is_arriving,
                lat: Some(lat + jitter_lat),
                lng: Some(lng + jitter_lng),
                angle: Some(angle + angle_jitter),
                distance_to_station: Some(dist),
                travel_time_secs: Some(travel_secs),
                station_name: Some(stop.name.into()),
                crowd_status: crowd,
                track_segment_index: Some(station_idx as i32),
                state_description: Some(desc),
            });
        }
        buses.sort_by_key(|b| b.station_index);
        buses
    }

    fn sim_segments(route: &SimRoute, rng: &mut Rng) -> Vec<RouteSegment> {
        let n = route.stops.len();
        (0..n.saturating_sub(1)).map(|i| {
            let s1 = &route.stops[i];
            let s2 = &route.stops[i + 1];
            let dist = ((s2.lat - s1.lat).powi(2) + (s2.lng - s1.lng).powi(2)).sqrt() * 111_000.0;
            let speed = 8.0 + rng.float() * 40.0;
            let cong = rng.pick(CONGESTION_LEVELS);
            RouteSegment { distance: Some(dist), speed: Some(speed), congestion: cong }
        }).collect()
    }

    fn sim_station_arrivals(route: &SimRoute, buses: &[BusPosition], rng: &mut Rng) -> Vec<StationArrival> {
        let n = route.stops.len();
        (1..=n).filter_map(|idx| {
            let arriving = buses.iter().filter(|b| b.station_index == idx as u32 && b.is_arriving).count() as u32;
            let leaving = buses.iter().filter(|b| b.station_index == idx as u32 && !b.is_arriving).count() as u32;
            let extra_arriving = if rng.range(0, 4) == 0 { 1 } else { 0 };
            let total_arriving = arriving + extra_arriving;
            if total_arriving == 0 && leaving == 0 { return None; }
            Some(StationArrival {
                station_index: idx as u32,
                station_name: Some(route.stops[idx - 1].name.into()),
                arriving_count: total_arriving,
                leaving_count: leaving,
            })
        }).collect()
    }

    fn sim_arrival_estimates(route: &SimRoute, target_order: u32, rng: &mut Rng) -> Vec<ArrivalDetail> {
        let n = route.stops.len() as u32;
        if target_order == 0 || target_order > n { return vec![]; }

        let bus_count = rng.range(1, 4);
        let mut estimates = Vec::new();
        let mut prev_away = 0u32;

        for _ in 0..bus_count {
            let away = prev_away + rng.range(1, 4) as u32;
            if away >= target_order { break; }
            let dist_per_stop = rng.range(300, 1200) as u32;
            let min_per_stop = rng.range(1, 3) as u32;
            estimates.push(ArrivalDetail {
                stations_away: away,
                minutes_away: away * min_per_stop + rng.range(0, 2) as u32,
                distance_m: away * dist_per_stop,
            });
            prev_away = away;
        }
        estimates
    }

    fn interpolate_track(route: &SimRoute) -> Vec<(f64, f64)> {
        let mut points = Vec::new();
        for i in 0..route.stops.len().saturating_sub(1) {
            let s1 = &route.stops[i];
            let s2 = &route.stops[i + 1];
            let steps = 4;
            for j in 0..steps {
                let t = j as f64 / steps as f64;
                points.push((
                    s1.lat + (s2.lat - s1.lat) * t,
                    s1.lng + (s2.lng - s1.lng) * t,
                ));
            }
        }
        let last = route.stops.last().unwrap();
        points.push((last.lat, last.lng));
        points
    }
}

#[async_trait]
impl BusDataProvider for DebugProvider {
    fn provider_name(&self) -> &str { "Debug" }
    fn city_name(&self) -> &str { "北京" }

    async fn nearby_stations(&self, lat: f64, lng: f64) -> Result<Vec<Station>, ProviderError> {
        let mut rng = Rng::from_time();
        let mut stations = Vec::new();

        for route in ROUTES {
            for stop in route.stops {
                let dlat = stop.lat - lat;
                let dlng = stop.lng - lng;
                let dist = ((dlat * dlat + dlng * dlng).sqrt() * 111_000.0) as u32;
                if dist < 2000 {
                    if stations.iter().any(|s: &Station| s.name == stop.name) {
                        continue;
                    }
                    stations.push(Station {
                        id: Some(stop.id.into()),
                        name: stop.name.into(),
                        alias: stop.alias.map(Into::into),
                        lat: stop.lat,
                        lng: stop.lng,
                        distance_m: dist,
                        same_name_count: rng.range(1, 4) as u32,
                    });
                }
            }
        }

        stations.sort_by_key(|s| s.distance_m);
        stations.truncate(10);
        Ok(stations)
    }

    async fn station_lines(
        &self,
        station: &str,
        _lat: f64,
        _lng: f64,
    ) -> Result<Vec<LineSummary>, ProviderError> {
        let mut rng = Rng::from_time();
        let mut lines = Vec::new();

        for route in ROUTES {
            let found = route.stops.iter().enumerate()
                .find(|(_, s)| s.name == station || s.id == station);
            let Some((idx, _)) = found else { continue; };

            let run_state = Self::sim_run_state(route, &mut rng);
            let arrival = match run_state {
                RunState::NotOperating | RunState::Stopped => ArrivalEstimate::NoService,
                RunState::NoRealtime => ArrivalEstimate::Unknown,
                RunState::Running => {
                    let v = rng.range(0, 10);
                    if v == 0 {
                        ArrivalEstimate::Arriving
                    } else if v <= 8 {
                        let stations_away = rng.range(1, 6) as u32;
                        ArrivalEstimate::Approaching {
                            stations_away,
                            minutes_away: Some(stations_away * 2 + rng.range(0, 3) as u32),
                            distance_m: Some(stations_away * rng.range(300, 900) as u32),
                        }
                    } else {
                        ArrivalEstimate::Unknown
                    }
                }
            };

            lines.push(LineSummary {
                id: Some(route.id.into()),
                name: route.name.into(),
                direction_id: route.dir_id.into(),
                endpoints: EndpointsView {
                    origin: route.origin.into(),
                    origin_alias: route.origin_alias.map(Into::into),
                    terminus: route.terminus.into(),
                    terminus_alias: route.terminus_alias.map(Into::into),
                },
                arrival,
                station_order: (idx + 1) as u32,
                run_state,
            });
        }

        Ok(lines)
    }

    async fn line_detail(&self, key: &str) -> Result<LineDetail, ProviderError> {
        let route = Self::find_route(key);
        let stops: Vec<LineStop> = route.stops.iter().enumerate().map(|(i, s)| {
            LineStop {
                id: Some(s.id.into()),
                name: s.name.into(),
                alias: s.alias.map(Into::into),
                order: (i + 1) as u32,
                lat: s.lat,
                lng: s.lng,
                status: s.status,
                track_index: Some((i * 4) as u32),
            }
        }).collect();

        let track_points = Self::interpolate_track(route);

        Ok(LineDetail {
            id: Some(route.id.into()),
            name: route.name.into(),
            direction_id: route.dir_id.into(),
            reverse_id: Some(route.reverse_id.into()),
            topology: RouteTopology {
                start: Terminal::named(route.origin),
                end: Terminal::named(route.terminus),
                stations: stops,
                track_points,
            },
            meta: LineMeta {
                first_service: ServiceTime::parse(route.first_service),
                last_service: ServiceTime::parse(route.last_service),
                fare: Fare::text(route.fare),
                company: Some(route.company.into()),
                contact: ContactInfo::Phone(route.contact.into()),
                notes: route.notes.map(Into::into),
            },
        })
    }

    async fn realtime(
        &self,
        key: &str,
        target_order: u32,
    ) -> Result<RealTimeData, ProviderError> {
        let route = Self::find_route(key);
        let mut rng = Rng::from_time();

        let run_state = Self::sim_run_state(route, &mut rng);
        if run_state == RunState::NotOperating || run_state == RunState::Stopped {
            return Ok(RealTimeData {
                run_state,
                plan_time: Some(route.first_service.into()),
                buses: vec![],
                station_arrivals: vec![],
                segments: Self::sim_segments(route, &mut rng),
                arrival_estimates: vec![],
            });
        }

        let buses = Self::sim_buses(route, &mut rng);
        let station_arrivals = Self::sim_station_arrivals(route, &buses, &mut rng);
        let segments = Self::sim_segments(route, &mut rng);
        let arrival_estimates = Self::sim_arrival_estimates(route, target_order, &mut rng);

        Ok(RealTimeData {
            run_state,
            plan_time: Some(route.first_service.into()),
            buses,
            station_arrivals,
            segments,
            arrival_estimates,
        })
    }

    async fn all_lines(&self) -> Result<Vec<BusRoute>, ProviderError> {
        Ok(ROUTES.iter().map(|r| BusRoute {
            id: Some(r.id.into()),
            name: r.name.into(),
            direction_id: r.dir_id.into(),
            terminals: (Terminal::named(r.origin), Terminal::named(r.terminus)),
            endpoints: EndpointsView {
                origin: r.origin.into(),
                origin_alias: r.origin_alias.map(Into::into),
                terminus: r.terminus.into(),
                terminus_alias: r.terminus_alias.map(Into::into),
            },
            company: Some(r.company.into()),
        }).collect())
    }
}
