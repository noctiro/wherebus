use serde::{Deserialize, Serialize};

use super::{EndpointsView, LineDetail, RealTimeData, RunState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub id: String,
    pub city_name: String,
    pub province: String,
    pub provider_name: String,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum NetworkState {
    #[default]
    Online,
    Degraded,
    Offline,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CacheCategory {
    Stations,
    StationLines,
    LineDetail,
    AllLines,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheStats {
    pub stations: usize,
    pub station_lines: usize,
    pub line_detail: usize,
    pub all_lines: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NearbyStationView {
    pub station_name: String,
    pub distance_m: i32,
    pub line_count: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NearbyLineView {
    pub line_name: String,
    #[serde(skip)]
    pub direction_id: String,
    pub endpoints: EndpointsView,
    pub arrival_state: String,
    pub run_state: Option<RunState>,
    pub stations_away: i32,
    pub minutes_away: i32,
    pub distance_m: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearbyGroup {
    pub station: NearbyStationView,
    pub lines: Vec<NearbyLineView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearbySnapshot {
    pub groups: Vec<NearbyGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineDetailSnapshot {
    pub detail: LineDetail,
    pub realtime: Option<RealTimeData>,
}

#[derive(Debug, Clone)]
pub enum QueryError {
    Message(String),
}

impl core::fmt::Display for QueryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Message(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for QueryError {}

impl From<String> for QueryError {
    fn from(value: String) -> Self {
        Self::Message(value)
    }
}

#[derive(Debug, Clone)]
pub enum ManageError {
    Message(String),
}

impl core::fmt::Display for ManageError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Message(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for ManageError {}

impl From<String> for ManageError {
    fn from(value: String) -> Self {
        Self::Message(value)
    }
}
