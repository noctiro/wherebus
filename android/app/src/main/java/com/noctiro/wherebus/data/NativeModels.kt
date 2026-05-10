package com.noctiro.wherebus.data

import kotlinx.serialization.Serializable

@Serializable
data class NativeLatLng(
    val lat: Double,
    val lng: Double,
)

@Serializable
data class NativeAppView(
    val show_welcome: Boolean = false,
    val welcome_show_permissions: Boolean = false,
    val welcome_location_granted: Boolean = false,
    val current_page: Int = 0,
    val show_line_detail: Boolean = false,
    val show_city_picker: Boolean = false,
    val show_cache_manager: Boolean = false,
    val nearby_loading: Boolean = false,
    val nearby_error: String = "",
    val nearby_items: List<NativeNearbyItemView> = emptyList(),
    val nearby_net_state: String = "Online",
    val search_query: String = "",
    val search_results: List<NativeLineCardView> = emptyList(),
    val search_loading: Boolean = false,
    val detail_line_name: String = "",
    val detail_direction: String = "",
    val detail_direction_label: String = "",
    val detail_can_switch: Boolean = false,
    val detail_comments: String = "",
    val detail_loading: Boolean = false,
    val detail_error: String = "",
    val detail_stations: List<NativeStationItemView> = emptyList(),
    val detail_first_time: String = "",
    val detail_last_time: String = "",
    val detail_price: String = "",
    val detail_company: String = "",
    val detail_phone: String = "",
    val detail_plan_time: String = "",
    val detail_run_state: String? = null,
    val settings_current_city: String = "",
    val settings_provider_name: String = "",
    val settings_location_auto: Boolean = true,
    val settings_auto_available: Boolean = false,
    val settings_auto_status: String = "",
    val settings_lat: String = "",
    val settings_lng: String = "",
    val settings_status: String = "",
    val settings_theme_index: Int = 0,
    val settings_cache_info: String = "",
    val settings_app_version: String = "",
    val settings_build_target: String = "",
    val settings_build_profile: String = "",
    val city_picker_query: String = "",
    val city_picker_cities: List<NativeCityItemView> = emptyList(),
    val cache_manager_categories: List<NativeCacheCategoryView> = emptyList(),
    val cache_manager_total: String = "",
)

@Serializable
data class NativeNearbyStationHeaderView(
    val station_name: String = "",
    val distance_m: Int = 0,
    val line_count: Int = 0,
)

@Serializable
data class NativeEndpointsView(
    val origin: String = "",
    val origin_alias: String? = null,
    val terminus: String = "",
    val terminus_alias: String? = null,
)

@Serializable
data class NativeNearbyLineSummaryView(
    val line_name: String = "",
    val endpoints: NativeEndpointsView = NativeEndpointsView(),
    val stations_away: Int = 0,
    val minutes_away: Int = 0,
    val distance_m: Int = 0,
    val arrival_state: String = "Unknown",
    val run_state: String? = null,
)

@Serializable
data class NativeNearbyItemView(
    val is_header: Boolean = false,
    val header: NativeNearbyStationHeaderView = NativeNearbyStationHeaderView(),
    val line: NativeNearbyLineSummaryView = NativeNearbyLineSummaryView(),
    val flat_index: Int = 0,
)

@Serializable
data class NativeLineCardView(
    val line_name: String = "",
    val endpoints: NativeEndpointsView = NativeEndpointsView(),
    val arrival_text: String = "",
    val arrival_distance: String = "",
    val station_name: String = "",
    val station_order: Int = 0,
    val company: String = "",
)

@Serializable
data class NativeBusItemView(
    val bus_id: String = "",
    val is_arriving: Boolean = false,
    val distance_m: Int = 0,
    val travel_time_secs: Int = 0,
    val crowd_level: String? = null,
    val state_desc: String = "",
)

@Serializable
data class NativeStationItemView(
    val name: String = "",
    val alias: String = "",
    val order: Int = 0,
    val is_current: Boolean = false,
    val is_nearest: Boolean = false,
    val distance_to_user_m: Int = 0,
    val has_bus: Boolean = false,
    val bus_info: String = "",
    val stop_type: String? = null,
    val congestion: String? = null,
    val prev_congestion: String? = null,
    val segment_distance_m: Int = 0,
    val segment_speed_kmh: Int = 0,
    val segment_time_secs: Int = 0,
    val buses: List<NativeBusItemView> = emptyList(),
)

@Serializable
data class NativeCityItemView(
    val name: String = "",
    val provider: String = "",
    val service_id: String = "",
    val is_header: Boolean = false,
)

@Serializable
data class NativeCacheCategoryView(
    val label: String = "",
    val count: Int = 0,
    val description: String = "",
)
