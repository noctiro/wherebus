use serde::{Deserialize, Serialize};

use crate::domain::{EndpointsView, NearbyLineView, NearbyStationView, NetworkState};
use crate::models::{CongestionLevel, CrowdLevel, RunState, StopStatus};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppView {
    pub show_welcome: bool,
    pub welcome_show_permissions: bool,
    pub welcome_location_granted: bool,
    pub current_page: i32,
    pub show_line_detail: bool,
    pub show_city_picker: bool,
    pub show_cache_manager: bool,
    pub nearby_loading: bool,
    pub nearby_error: String,
    pub nearby_items: Vec<NearbyItemView>,
    pub nearby_net_state: NetworkState,
    pub search_query: String,
    pub search_results: Vec<LineCardView>,
    pub search_loading: bool,
    pub detail_line_name: String,
    pub detail_direction: String,
    pub detail_direction_label: String,
    pub detail_can_switch: bool,
    pub detail_comments: String,
    pub detail_loading: bool,
    pub detail_error: String,
    pub detail_stations: Vec<StationItemView>,
    pub detail_first_time: String,
    pub detail_last_time: String,
    pub detail_price: String,
    pub detail_company: String,
    pub detail_phone: String,
    pub detail_plan_time: String,
    pub detail_run_state: Option<RunState>,
    pub settings_current_city: String,
    pub settings_provider_name: String,
    pub settings_location_auto: bool,
    pub settings_auto_available: bool,
    pub settings_auto_status: String,
    pub settings_lat: String,
    pub settings_lng: String,
    pub settings_status: String,
    pub settings_theme_index: i32,
    pub settings_cache_info: String,
    pub settings_app_version: String,
    pub settings_build_target: String,
    pub settings_build_profile: String,
    pub city_picker_query: String,
    pub city_picker_cities: Vec<CityItemView>,
    pub cache_manager_categories: Vec<CacheCategoryView>,
    pub cache_manager_total: String,
}

pub type NearbyStationHeaderView = NearbyStationView;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NearbyItemView {
    pub is_header: bool,
    pub header: NearbyStationHeaderView,
    pub line: NearbyLineView,
    pub flat_index: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LineCardView {
    pub line_name: String,
    #[serde(skip)]
    pub direction_id: String,
    pub endpoints: EndpointsView,
    pub arrival_text: String,
    pub arrival_distance: String,
    pub station_name: String,
    pub station_order: i32,
    pub company: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusItemView {
    pub bus_id: String,
    pub is_arriving: bool,
    pub distance_m: i32,
    pub travel_time_secs: i32,
    pub crowd_level: Option<CrowdLevel>,
    pub state_desc: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StationItemView {
    pub name: String,
    pub alias: String,
    pub order: i32,
    pub is_current: bool,
    pub is_nearest: bool,
    pub distance_to_user_m: i32,
    pub has_bus: bool,
    pub bus_info: String,
    pub stop_type: Option<StopStatus>,
    pub congestion: Option<CongestionLevel>,
    pub prev_congestion: Option<CongestionLevel>,
    pub segment_distance_m: i32,
    pub segment_speed_kmh: i32,
    pub segment_time_secs: i32,
    pub buses: Vec<BusItemView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CityItemView {
    pub name: String,
    pub provider: String,
    pub service_id: String,
    pub is_header: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheCategoryView {
    pub label: String,
    pub count: i32,
    pub description: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CacheStatsView {
    pub stations: usize,
    pub station_lines: usize,
    pub line_detail: usize,
    pub all_lines: usize,
}
