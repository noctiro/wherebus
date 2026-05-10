use crate::models::{AppConfig, LineSummary};
use serde::{Deserialize, Serialize};

use super::view::{CacheStatsView, CityItemView, NearbyItemView};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapData {
    pub config: AppConfig,
    pub provider_name: String,
    pub city_label: String,
    pub location_permission: bool,
    pub city_picker_cities: Vec<CityItemView>,
    pub cache_stats: CacheStatsView,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSwitchData {
    pub provider_name: String,
    pub city_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearbyData {
    pub items: Vec<NearbyItemView>,
    pub nearby_lines: Vec<Option<LineSummary>>,
}
