use serde::Deserialize;

// ─── encryptedNearlines 解密后 ───
// 返回: { nearLines: [{
//   sId, sn (站名), lat, lng, distance,
//   lines: [{
//     line: { lineId, name, direction (0/1), desc, startSn (起点站名), endSn (终点站名),
//             firstTime, lastTime, price, state, stationsNum, ... },
//     targetStation: { order, sId, sn, distanceToSp },
//     nextStation: { order, sId, sn, distanceToSp },
//   }]
// }] }

#[derive(Debug, Clone, Deserialize)]
pub struct NearbyResponse {
    #[serde(rename = "nearLines", default)]
    pub near_lines: Vec<NearStation>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NearStation {
    #[serde(rename = "sId", default)]
    pub s_id: String,
    #[serde(rename = "sn", default)]
    pub station_name: String,
    #[serde(default)]
    pub lat: f64,
    #[serde(default)]
    pub lng: f64,
    #[serde(default)]
    pub distance: f64,
    #[serde(rename = "lines", default)]
    pub lines: Vec<NearLineWrapper>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NearLineWrapper {
    #[serde(default)]
    pub line: NearLineInfo,
    #[serde(rename = "targetStation", default)]
    pub target_station: Option<NearTargetStation>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct NearLineInfo {
    #[serde(rename = "lineId", default)]
    pub line_id: String,
    #[serde(default)]
    pub name: String,
    #[allow(dead_code)]
    #[serde(default)]
    pub direction: u8,
    #[serde(default)]
    pub desc: String,
    #[serde(default)]
    pub state: i32,
    /// 起点站名
    #[serde(rename = "startSn", default)]
    pub start_sn: String,
    /// 终点站名
    #[serde(rename = "endSn", default)]
    pub end_sn: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NearTargetStation {
    #[serde(default)]
    pub order: u32,
    #[serde(rename = "distanceToSp", default)]
    pub distance_to_sp: u32,
}

// ─── encryptedLineDetail 解密后 ───
// 返回: { buses: [{ busId, order, distanceToSc, travelTime, timeStr, state, lat, lng }] }

#[derive(Debug, Clone, Deserialize)]
pub struct LineDetailResponse {
    #[serde(default)]
    pub buses: Vec<DetailBus>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DetailBus {
    #[serde(rename = "busId", default)]
    pub bus_id: String,
    #[serde(default)]
    pub order: u32,
    #[serde(rename = "distanceToSc", default)]
    pub distance_to_sc: f64,
    #[serde(rename = "travelTime", default)]
    pub travel_time: f64,
    #[serde(rename = "timeStr", default)]
    pub time_str: String,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub lat: Option<f64>,
    #[serde(default)]
    pub lng: Option<f64>,
}

// ─── lineRoute ───
// 返回: { line: { lineNo, firstTime, lastTime, price, company },
//         stations: [{ sId, sn (站名), lat, lng, order }] }

#[derive(Debug, Clone, Deserialize)]
pub struct LineRouteResponse {
    #[serde(default)]
    pub line: Option<RouteLine>,
    #[serde(default)]
    pub stations: Vec<RouteStation>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RouteLine {
    #[serde(rename = "lineNo", default)]
    pub line_no: String,
    #[serde(rename = "firstTime", default)]
    pub first_time: String,
    #[serde(rename = "lastTime", default)]
    pub last_time: String,
    #[serde(default)]
    pub price: String,
    #[serde(default)]
    pub company: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RouteStation {
    #[serde(rename = "sId", default)]
    pub s_id: String,
    #[serde(rename = "sn", default)]
    pub station_name: String,
    #[serde(default)]
    pub lat: f64,
    #[serde(default)]
    pub lng: f64,
    #[serde(default)]
    pub order: u32,
}

// ─── cityLineList ───
// 返回: { allLines: { all: [{ lineId, lineName, startStopName, endStopName }] } }

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "allLines", default)]
    pub all_lines: SearchAllLines,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SearchAllLines {
    #[serde(default)]
    pub all: Vec<SearchLine>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchLine {
    #[serde(rename = "lineId", default)]
    pub line_id: String,
    #[serde(rename = "lineName", default)]
    pub line_name: String,
    /// 起点站名
    #[serde(rename = "startStopName", default)]
    pub start_stop_name: String,
    /// 终点站名
    #[serde(rename = "endStopName", default)]
    pub end_stop_name: String,
}
