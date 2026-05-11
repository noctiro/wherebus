package com.noctiro.wherebus.ui

import com.noctiro.wherebus.data.NativeAppView
import com.noctiro.wherebus.data.NativeBusItemView
import com.noctiro.wherebus.data.NativeCacheCategoryView
import com.noctiro.wherebus.data.NativeCityItemView
import com.noctiro.wherebus.data.NativeLineCardView
import com.noctiro.wherebus.data.NativeNearbyItemView
import com.noctiro.wherebus.data.NativeStationItemView
import com.noctiro.wherebus.domain.AppConfig
import com.noctiro.wherebus.domain.ArrivalState
import com.noctiro.wherebus.domain.BusItemUi
import com.noctiro.wherebus.domain.CacheCategory
import com.noctiro.wherebus.domain.CacheStats
import com.noctiro.wherebus.domain.CongestionLevel
import com.noctiro.wherebus.domain.CrowdLevel
import com.noctiro.wherebus.domain.LatLng
import com.noctiro.wherebus.domain.LineCardUi
import com.noctiro.wherebus.domain.LineDetailSnapshot
import com.noctiro.wherebus.domain.LineDetailUi
import com.noctiro.wherebus.domain.LocationMode
import com.noctiro.wherebus.domain.NearbyGroup
import com.noctiro.wherebus.domain.NearbyLineView
import com.noctiro.wherebus.domain.NearbyStationView
import com.noctiro.wherebus.domain.NetworkState
import com.noctiro.wherebus.domain.RealtimeData
import com.noctiro.wherebus.domain.RunState
import com.noctiro.wherebus.domain.ServiceInfo
import com.noctiro.wherebus.domain.StationItemUi
import com.noctiro.wherebus.domain.StopStatus
import com.noctiro.wherebus.domain.ThemeMode

internal fun NativeAppView.toUiState(feedbackMessage: String?): WhereBusUiState {
    val config = AppConfig(
        serviceId = "",
        onboardingDone = !show_welcome,
        locationMode = if (settings_location_auto) LocationMode.Auto else LocationMode.Manual,
        manualLocation = settings_lat.toDoubleOrNull()?.let { lat ->
            settings_lng.toDoubleOrNull()?.let { lng -> LatLng(lat, lng) }
        },
        autoLocation = null,
        themeMode = when (settings_theme_index) {
            1 -> ThemeMode.Light
            2 -> ThemeMode.Dark
            else -> ThemeMode.System
        },
    )

    val nearbyGroups = nearby_items.toNearbyGroups()
    val detailSnapshot = if (detail_line_name.isNotBlank() || detail_stations.isNotEmpty()) {
        LineDetailSnapshot(
            detail = LineDetailUi(
                lineName = detail_line_name,
                directionLabel = detail_direction_label,
                comments = detail_comments,
                firstTime = detail_first_time,
                lastTime = detail_last_time,
                price = detail_price,
                company = detail_company,
                phone = detail_phone,
                planTime = detail_plan_time,
                runState = detail_run_state.toRunState(),
                stations = detail_stations.map { it.toDomain() },
            ),
            realtime = RealtimeData(
                hasRealtime = detail_stations.any { it.buses.isNotEmpty() },
                runState = detail_run_state.toRunState(),
                planTime = detail_plan_time,
                buses = detail_stations.flatMap { station -> station.buses.map { it.toDomain() } },
            ),
        )
    } else {
        null
    }

    return WhereBusUiState(
        initialSync = false,
        showWelcome = show_welcome,
        currentPage = when (current_page) {
            1 -> RootPage.Settings
            else -> RootPage.Nearby
        },
        cityLabel = settings_current_city,
        providerName = settings_provider_name,
        config = config,
        locationPermissionGranted = welcome_location_granted,
        networkState = when (nearby_net_state) {
            "Degraded" -> NetworkState.Degraded
            "Offline" -> NetworkState.Offline
            else -> NetworkState.Online
        },
        nearbyLoading = nearby_loading,
        nearbyError = nearby_error,
        nearbyGroups = nearbyGroups,
        searchQuery = search_query,
        searchLoading = search_loading,
        searchResults = search_results.map { it.toDomain() },
        detailLoading = detail_loading,
        detailError = detail_error,
        detailCanSwitch = detail_can_switch,
        showDetail = show_line_detail,
        detail = detailSnapshot,
        showCityPicker = show_city_picker,
        cityPickerQuery = city_picker_query,
        cityPickerGroups = city_picker_cities.toCityGroups(),
        cacheCategories = cache_manager_categories.mapIndexed { index, item ->
            CacheCategoryUi(
                category = when (index) {
                    0 -> CacheCategory.Stations
                    1 -> CacheCategory.StationLines
                    2 -> CacheCategory.LineDetail
                    else -> CacheCategory.AllLines
                },
                label = item.label,
                count = item.count,
                description = item.description,
            )
        },
        showCacheManager = show_cache_manager,
        cacheStats = cache_manager_categories.toCacheStats(),
        manualLatText = settings_lat,
        manualLngText = settings_lng,
        feedbackMessage = feedbackMessage,
    )
}

private fun List<NativeNearbyItemView>.toNearbyGroups(): List<NearbyGroup> {
    val groups = mutableListOf<NearbyGroup>()
    var currentHeader: NearbyStationView? = null
    var currentLines = mutableListOf<NearbyLineView>()

    for (item in this) {
        if (item.is_header) {
            if (currentHeader != null) {
                groups += NearbyGroup(currentHeader, currentLines.toList())
            }
            currentHeader = NearbyStationView(
                name = item.header.station_name,
                distanceMeters = item.header.distance_m,
                lineCount = item.header.line_count,
            )
            currentLines = mutableListOf()
        } else {
            currentLines += NearbyLineView(
                lineName = item.line.line_name,
                origin = item.line.endpoints.origin,
                originAlias = item.line.endpoints.origin_alias,
                terminus = item.line.endpoints.terminus,
                terminusAlias = item.line.endpoints.terminus_alias,
                arrivalState = item.line.arrival_state.toArrivalState(),
                runState = item.line.run_state.toRunState(),
                stationsAway = item.line.stations_away,
                minutesAway = item.line.minutes_away,
                distanceMeters = item.line.distance_m,
                flatIndex = item.flat_index,
            )
        }
    }

    if (currentHeader != null) {
        groups += NearbyGroup(currentHeader, currentLines.toList())
    }
    return groups
}

private fun NativeLineCardView.toDomain(): LineCardUi = LineCardUi(
    lineName = line_name,
    origin = endpoints.origin,
    originAlias = endpoints.origin_alias,
    terminus = endpoints.terminus,
    terminusAlias = endpoints.terminus_alias,
    arrivalText = arrival_text,
    arrivalDistance = arrival_distance,
    stationName = station_name,
    stationOrder = station_order,
    company = company,
)

private fun NativeCityItemView.toDomain(): ServiceInfo = ServiceInfo(
    id = service_id,
    cityName = name,
    province = "",
    providerName = provider,
)

private fun List<NativeCityItemView>.toCityGroups(): List<CityGroup> {
    val groups = mutableListOf<CityGroup>()
    var currentRegion = ""
    var currentServices = mutableListOf<ServiceInfo>()

    for (item in this) {
        if (item.is_header) {
            if (currentRegion.isNotBlank()) {
                groups += CityGroup(currentRegion, currentServices.toList())
            }
            currentRegion = item.name
            currentServices = mutableListOf()
        } else {
            currentServices += item.toDomain()
        }
    }
    if (currentRegion.isNotBlank() && currentServices.isNotEmpty()) {
        groups += CityGroup(currentRegion, currentServices.toList())
    }
    return groups
}

private fun NativeStationItemView.toDomain(): StationItemUi = StationItemUi(
    name = name,
    alias = alias,
    order = order,
    isCurrent = is_current,
    isNearest = is_nearest,
    distanceToUserMeters = distance_to_user_m,
    hasBus = has_bus,
    busInfo = bus_info,
    stopStatus = stop_type.toStopStatus(),
    congestion = congestion.toCongestion(),
    prevCongestion = prev_congestion.toCongestion(),
    segmentDistanceMeters = segment_distance_m,
    segmentSpeedKmh = segment_speed_kmh,
    segmentTimeSeconds = segment_time_secs,
    buses = buses.map { it.toDomain() },
)

private fun NativeBusItemView.toDomain() = BusItemUi(
    busId = bus_id,
    isArriving = is_arriving,
    distanceMeters = distance_m,
    travelTimeSeconds = travel_time_secs,
    crowdLevel = crowd_level.toCrowdLevel(),
    stateDescription = state_desc,
)

private fun List<NativeCacheCategoryView>.toCacheStats(): CacheStats {
    fun countAt(index: Int): Int = getOrNull(index)?.count ?: 0
    return CacheStats(
        stations = countAt(0),
        stationLines = countAt(1),
        lineDetail = countAt(2),
        allLines = countAt(3),
    )
}

private fun String?.toRunState(): RunState? = when (this) {
    "Running" -> RunState.Running
    "NotOperating" -> RunState.NotOperating
    "Stopped" -> RunState.Stopped
    "NoRealtime" -> RunState.NoRealtime
    else -> null
}

private fun String.toArrivalState(): ArrivalState = when (this) {
    "Arriving" -> ArrivalState.Arriving
    "Approaching" -> ArrivalState.Approaching
    "NoService" -> ArrivalState.NoService
    else -> ArrivalState.Unknown
}

private fun String?.toCrowdLevel(): CrowdLevel? = when (this) {
    "Spacious" -> CrowdLevel.Spacious
    "Normal" -> CrowdLevel.Normal
    "Crowded" -> CrowdLevel.Crowded
    "Full" -> CrowdLevel.Full
    else -> null
}

private fun String?.toStopStatus(): StopStatus? = when (this) {
    "NotStopping" -> StopStatus.NotStopping
    "BoardOnly" -> StopStatus.BoardOnly
    "AlightOnly" -> StopStatus.AlightOnly
    "Temporary" -> StopStatus.Temporary
    "OnDemand" -> StopStatus.OnDemand
    "Express" -> StopStatus.Express
    else -> null
}

private fun String?.toCongestion(): CongestionLevel? = when (this) {
    "Smooth" -> CongestionLevel.Smooth
    "Slow" -> CongestionLevel.Slow
    "Congested" -> CongestionLevel.Congested
    else -> null
}
