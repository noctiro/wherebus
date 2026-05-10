use serde::Deserialize;
use serde::de::Deserializer;

fn string_or_float_default<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrFloat {
        Str(String),
        Float(f64),
        Int(i64),
        Null,
    }
    match StringOrFloat::deserialize(deserializer) {
        Ok(StringOrFloat::Str(s)) => Ok(s),
        Ok(StringOrFloat::Float(f)) => Ok(f.to_string()),
        Ok(StringOrFloat::Int(i)) => Ok(i.to_string()),
        Ok(StringOrFloat::Null) | Err(_) => Ok(String::new()),
    }
}

// ─── CMD 106: 附近站点列表 ───
// 返回: { data: [{ name, lat, lon, dis, sameNum }], status, msg }

#[derive(Debug, Clone, Deserialize)]
pub struct NearbyStationsResponse {
    #[serde(default)]
    pub data: Vec<NearbyStation>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NearbyStation {
    pub name: String,
    #[serde(default, deserialize_with = "string_or_float_default")]
    pub lat: String,
    #[serde(default, deserialize_with = "string_or_float_default")]
    pub lon: String,
    #[serde(default)]
    pub dis: u32,
    #[serde(rename = "sameNum", default)]
    pub same_num: u32,
}

// ─── CMD 115: 站点线路实时信息 ───
// 返回: { data: [{ lineName, upperOrDown, neartext, neardis, stationOrder }] }
// 注意: upperOrDown 为方向码 "1"(上行) / "2"(下行)

#[derive(Debug, Clone, Deserialize)]
pub struct StationLinesResponse {
    #[serde(default)]
    pub data: Vec<StationLine>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StationLine {
    #[serde(rename = "lineName")]
    pub line_name: String,
    /// 方向码: "1"(上行) / "2"(下行)
    #[serde(rename = "upperOrDown", default)]
    pub direction: String,
    #[serde(default)]
    pub neartext: String,
    #[serde(default)]
    pub neardis: String,
    #[serde(default)]
    pub neartime: String,
    #[serde(default)]
    pub nearnum: i32,
    #[serde(rename = "stationOrder", default)]
    pub station_order: u32,
}

// ─── CMD 103: 线路站点列表 ───
// 返回: { routeName, upperOrDown ("1"/"2"), commonts, beginTime, endTime, planTime,
//         firstLast: [{ first, last }],
//         data: [{ showName, stationOrder, station_lat, station_lon, stationsStatus, StationNihePointIndex }],
//         nihelist: [{ lat, lng }] }
// 注意: upperOrDown 在此接口为数字 "1"(上行) / "2"(下行)

#[derive(Debug, Clone, Deserialize)]
pub struct LineStationsResponse {
    #[serde(rename = "routeName", default)]
    pub route_name: String,
    #[serde(rename = "upperOrDown", default)]
    pub direction: String,
    #[serde(rename = "commonts", default)]
    pub comments: String,
    #[serde(rename = "firstLast", default)]
    pub first_last: Vec<FirstLast>,
    #[serde(default)]
    pub data: Vec<LineStation>,
    #[serde(default)]
    pub nihelist: Vec<NihePoint>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FirstLast {
    #[serde(default)]
    pub first: String,
    #[serde(default)]
    pub last: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LineStation {
    #[serde(rename = "showName", default)]
    pub show_name: String,
    #[serde(rename = "stationOrder", default)]
    pub station_order: u32,
    #[serde(
        rename = "station_lat",
        default,
        deserialize_with = "string_or_float_default"
    )]
    pub lat: String,
    #[serde(
        rename = "station_lon",
        default,
        deserialize_with = "string_or_float_default"
    )]
    pub lng: String,
    #[serde(rename = "stationsStatus", default)]
    pub status: u8,
    #[serde(rename = "StationNihePointIndex", default)]
    pub nihe_point_index: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NihePoint {
    #[serde(default, deserialize_with = "string_or_float_default")]
    pub lat: String,
    #[serde(default, deserialize_with = "string_or_float_default")]
    pub lng: String,
}

// ─── CMD 104: 实时车辆位置 ───
// 返回: { runState, hasReal, planTime,
//         data: [{ index, arrive, come, stationName }],
//         list: [{ index, busNumber, statusType, stationName, bus_lat, bus_lng, nihePointIndex, angle, busToStationNiheDistance }],
//         dislist: [{ d }], speedlist: [{ speed, co }] }

#[derive(Debug, Clone, Deserialize)]
pub struct RealTimeResponse {
    #[serde(rename = "runState", default)]
    pub run_state: u8,
    #[serde(rename = "hasReal", default)]
    pub has_real: u8,
    #[serde(rename = "planTime", default)]
    pub plan_time: Option<String>,
    #[serde(default)]
    pub data: Vec<StationBusInfo>,
    #[serde(default)]
    pub list: Vec<BusInfo>,
    #[serde(default)]
    pub dislist: Vec<DisInfo>,
    #[serde(default)]
    pub speedlist: Vec<SpeedInfo>,
    #[serde(rename = "routeOnStationRTimeInfoList", default)]
    pub rtime_list: Vec<RTimeInfo>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RTimeInfo {
    #[serde(rename = "busToStationCount", default)]
    pub count: i32,
    #[serde(rename = "busToStationTime", default)]
    pub time: f64,
    #[serde(rename = "busToStationDistance", default)]
    pub distance: u32,
    #[allow(dead_code)]
    #[serde(rename = "busToStationTips", default)]
    pub tips: String,
    #[allow(dead_code)]
    #[serde(rename = "busToStationTimeTips", default)]
    pub time_tips: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StationBusInfo {
    #[serde(default)]
    pub index: u32,
    #[serde(default)]
    pub arrive: u32,
    #[serde(default)]
    pub come: u32,
    #[serde(rename = "stationName", default)]
    pub station_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BusInfo {
    #[serde(default)]
    pub index: u32,
    #[serde(rename = "busNumber", default)]
    pub bus_number: String,
    #[serde(rename = "statusType", default)]
    pub status_type: String,
    #[serde(rename = "stationName", default)]
    pub station_name: String,
    #[serde(
        rename = "bus_lat",
        default,
        deserialize_with = "string_or_float_default"
    )]
    pub lat: String,
    #[serde(
        rename = "bus_lng",
        default,
        deserialize_with = "string_or_float_default"
    )]
    pub lng: String,
    #[serde(rename = "nihePointIndex", default)]
    pub nihe_point_index: i32,
    #[serde(default)]
    pub angle: f64,
    #[serde(rename = "busToStationNiheDistance", default)]
    pub bus_to_station_distance: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DisInfo {
    #[serde(default)]
    pub d: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpeedInfo {
    #[serde(default)]
    pub speed: f64,
    #[serde(default)]
    pub co: String,
}

// ─── CMD 119: 所有线路列表 ───
// 返回: { buslines: [{ lineName, upperOrDown ("1"/"2"), from, to, company, shortName, ... }] }
// 注意: upperOrDown 为 "1"(上行) / "2"(下行)，from/to 为起终点站名

#[derive(Debug, Clone, Deserialize)]
pub struct AllLinesResponse {
    #[serde(default)]
    pub buslines: Vec<BusLine>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BusLine {
    #[serde(rename = "lineName")]
    pub line_name: String,
    /// "1"(上行) / "2"(下行)
    #[serde(rename = "upperOrDown", default)]
    pub direction: String,
    /// 起点站名
    #[serde(default)]
    pub from: String,
    /// 终点站名
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub company: String,
}

// ─── 城市配置 ───

#[derive(Debug, Clone, serde::Serialize, Deserialize)]
pub struct CityConfig {
    pub name: String,
    pub key: String,
}
