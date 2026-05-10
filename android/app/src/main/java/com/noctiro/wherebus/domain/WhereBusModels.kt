package com.noctiro.wherebus.domain

import java.util.Locale

data class ServiceInfo(
    val id: String,
    val cityName: String,
    val province: String,
    val providerName: String,
)

enum class NetworkState {
    Online,
    Degraded,
    Offline,
}

enum class CacheCategory {
    Stations,
    StationLines,
    LineDetail,
    AllLines,
}

data class CacheStats(
    val stations: Int = 0,
    val stationLines: Int = 0,
    val lineDetail: Int = 0,
    val allLines: Int = 0,
) {
    fun total(): Int = stations + stationLines + lineDetail + allLines
}

data class LatLng(
    val lat: Double,
    val lng: Double,
) {
    fun label(): String = "${lat.formatCoordinate()}, ${lng.formatCoordinate()}"
}

enum class LocationMode {
    Auto,
    Manual,
}

enum class ThemeMode {
    System,
    Light,
    Dark,
}

data class AppConfig(
    val serviceId: String = "",
    val onboardingDone: Boolean = false,
    val locationMode: LocationMode = LocationMode.Auto,
    val manualLocation: LatLng? = null,
    val autoLocation: LatLng? = null,
    val themeMode: ThemeMode = ThemeMode.System,
) {
    fun effectiveLocation(): LatLng? = when (locationMode) {
        LocationMode.Auto -> autoLocation ?: manualLocation
        LocationMode.Manual -> manualLocation ?: autoLocation
    }
}

enum class RunState {
    Running,
    NotOperating,
    Stopped,
    NoRealtime,
}

enum class CrowdLevel {
    Spacious,
    Normal,
    Crowded,
    Full,
}

enum class StopStatus {
    NotStopping,
    BoardOnly,
    AlightOnly,
    Temporary,
    OnDemand,
    Express,
}

enum class CongestionLevel {
    Smooth,
    Slow,
    Congested,
}

data class NearbyStationView(
    val name: String,
    val distanceMeters: Int,
    val lineCount: Int,
)

enum class ArrivalState {
    Arriving,
    Approaching,
    NoService,
    Unknown,
}

data class NearbyLineView(
    val lineName: String,
    val origin: String,
    val originAlias: String? = null,
    val terminus: String,
    val terminusAlias: String? = null,
    val arrivalState: ArrivalState = ArrivalState.Unknown,
    val runState: RunState?,
    val stationsAway: Int = 0,
    val minutesAway: Int = 0,
    val distanceMeters: Int = 0,
    val flatIndex: Int = 0,
)

data class NearbyGroup(
    val station: NearbyStationView,
    val lines: List<NearbyLineView>,
)

data class LineCardUi(
    val lineName: String,
    val origin: String,
    val originAlias: String? = null,
    val terminus: String,
    val terminusAlias: String? = null,
    val arrivalText: String,
    val arrivalDistance: String,
    val stationName: String,
    val stationOrder: Int,
    val company: String,
)

data class BusItemUi(
    val busId: String,
    val isArriving: Boolean,
    val distanceMeters: Int,
    val travelTimeSeconds: Int,
    val crowdLevel: CrowdLevel?,
    val stateDescription: String,
)

data class StationItemUi(
    val name: String,
    val alias: String,
    val order: Int,
    val isCurrent: Boolean,
    val isNearest: Boolean,
    val distanceToUserMeters: Int,
    val hasBus: Boolean,
    val busInfo: String,
    val stopStatus: StopStatus?,
    val congestion: CongestionLevel?,
    val prevCongestion: CongestionLevel?,
    val segmentDistanceMeters: Int,
    val segmentSpeedKmh: Int,
    val segmentTimeSeconds: Int,
    val buses: List<BusItemUi>,
)

data class LineDetailUi(
    val lineName: String,
    val directionLabel: String,
    val comments: String,
    val firstTime: String,
    val lastTime: String,
    val price: String,
    val company: String,
    val phone: String,
    val planTime: String,
    val runState: RunState?,
    val stations: List<StationItemUi>,
)

data class RealtimeData(
    val hasRealtime: Boolean,
    val runState: RunState?,
    val planTime: String,
    val buses: List<BusItemUi>,
)

data class LineDetailSnapshot(
    val detail: LineDetailUi,
    val realtime: RealtimeData? = null,
)

data class BootstrapSnapshot(
    val config: AppConfig,
    val providerName: String,
    val cityLabel: String,
    val locationPermissionGranted: Boolean,
    val cityPickerCities: List<ServiceInfo>,
    val cacheStats: CacheStats,
)

fun runStateLabel(state: RunState): String = when (state) {
    RunState.Running -> "运行中"
    RunState.NotOperating -> "未运营"
    RunState.Stopped -> "已停运"
    RunState.NoRealtime -> "无实时"
}

fun crowdLabel(level: CrowdLevel): String = when (level) {
    CrowdLevel.Spacious -> "宽松"
    CrowdLevel.Normal -> "正常"
    CrowdLevel.Crowded -> "拥挤"
    CrowdLevel.Full -> "满载"
}

fun stopStatusLabel(status: StopStatus): String = when (status) {
    StopStatus.NotStopping -> "不停靠"
    StopStatus.BoardOnly -> "只上车"
    StopStatus.AlightOnly -> "只下车"
    StopStatus.Temporary -> "临时站"
    StopStatus.OnDemand -> "招呼站"
    StopStatus.Express -> "快线站"
}

fun congestionLabel(level: CongestionLevel): String = when (level) {
    CongestionLevel.Smooth -> "畅通"
    CongestionLevel.Slow -> "缓行"
    CongestionLevel.Congested -> "拥堵"
}

fun CacheCategory.label(): String = when (this) {
    CacheCategory.Stations -> "附近站点"
    CacheCategory.StationLines -> "站点线路"
    CacheCategory.LineDetail -> "线路详情"
    CacheCategory.AllLines -> "全部线路"
}

fun ThemeMode.label(): String = when (this) {
    ThemeMode.System -> "跟随系统"
    ThemeMode.Light -> "浅色"
    ThemeMode.Dark -> "深色"
}

fun NetworkState.label(): String = when (this) {
    NetworkState.Online -> "在线"
    NetworkState.Degraded -> "受限"
    NetworkState.Offline -> "离线"
}

fun Int.formatDistanceCompact(): String = when {
    this <= 0 -> "-"
    this >= 1000 -> String.format(Locale.US, "%.1f km", this / 1000.0)
    else -> "${this} m"
}

fun Int.formatDurationCompact(): String = when {
    this <= 0 -> "-"
    this < 60 -> "${this}s"
    this < 3600 -> "${this / 60} min"
    else -> String.format(Locale.US, "%.1f h", this / 3600.0)
}

fun Int.formatSpeedCompact(): String = when {
    this <= 0 -> "-"
    else -> "${this} km/h"
}

private fun Double.formatCoordinate(): String = String.format(Locale.US, "%.4f", this)
