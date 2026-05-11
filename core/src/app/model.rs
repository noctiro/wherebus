use crate::domain::{LineDetailSnapshot, NetworkState};
use crate::models::{AppConfig, BusRoute, LineSummary};

use super::view::{CacheStatsView, CityItemView, LineCardView, NearbyItemView, StationItemView};

#[derive(Debug, Clone, Default)]
pub struct SelectedLine {
    pub name: String,
    pub direction_id: String,
    pub reverse_id: Option<String>,
    pub target_order: u32,
}

#[derive(Debug, Clone, Default)]
pub struct Model {
    pub config: AppConfig,
    pub provider_name: String,
    pub city_label: String,
    pub location_permission: bool,
    pub current_page: i32,
    pub show_line_detail: bool,
    pub show_city_picker: bool,
    pub show_cache_manager: bool,
    pub show_welcome: bool,
    pub welcome_show_permissions: bool,
    pub welcome_location_granted: bool,
    pub nearby_loading: bool,
    pub nearby_error: String,
    pub nearby_items: Vec<NearbyItemView>,
    pub nearby_net_state: NetworkState,
    pub nearby_lines: Vec<Option<LineSummary>>,
    pub search_loading: bool,
    pub search_query: String,
    pub search_results: Vec<LineCardView>,
    pub search_lookup: Vec<SelectedLine>,
    pub all_routes: Vec<BusRoute>,
    pub detail_line_name: String,
    pub detail_loading: bool,
    pub detail_error: String,
    pub detail_stations: Vec<StationItemView>,
    pub detail_snapshot: Option<LineDetailSnapshot>,
    pub current_line: Option<SelectedLine>,
    pub settings_auto_available: bool,
    pub settings_auto_status: String,
    pub settings_status: String,
    pub city_picker_query: String,
    pub city_picker_all: Vec<CityItemView>,
    pub city_picker_cities: Vec<CityItemView>,
    pub cache_stats: CacheStatsView,
}
