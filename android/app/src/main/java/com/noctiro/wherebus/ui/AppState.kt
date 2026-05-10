package com.noctiro.wherebus.ui

import com.noctiro.wherebus.domain.AppConfig
import com.noctiro.wherebus.domain.CacheCategory
import com.noctiro.wherebus.domain.CacheStats
import com.noctiro.wherebus.domain.LineCardUi
import com.noctiro.wherebus.domain.LineDetailSnapshot
import com.noctiro.wherebus.domain.LocationMode
import com.noctiro.wherebus.domain.NearbyGroup
import com.noctiro.wherebus.domain.NetworkState
import com.noctiro.wherebus.domain.ServiceInfo
import com.noctiro.wherebus.domain.ThemeMode

enum class RootPage {
    Nearby,
    Settings,
}

data class WhereBusUiState(
    val loading: Boolean = true,
    val showWelcome: Boolean = false,
    val currentPage: RootPage = RootPage.Nearby,
    val showSearch: Boolean = false,
    val cityLabel: String = "",
    val providerName: String = "",
    val config: AppConfig = AppConfig(),
    val locationPermissionGranted: Boolean = false,
    val networkState: NetworkState = NetworkState.Online,
    val nearbyLoading: Boolean = false,
    val nearbyError: String = "",
    val nearbyGroups: List<NearbyGroup> = emptyList(),
    val searchQuery: String = "",
    val searchLoading: Boolean = false,
    val searchResults: List<LineCardUi> = emptyList(),
    val detailLoading: Boolean = false,
    val detailError: String = "",
    val detailCanSwitch: Boolean = false,
    val showDetail: Boolean = false,
    val detail: LineDetailSnapshot? = null,
    val showCityPicker: Boolean = false,
    val cityPickerQuery: String = "",
    val cityPickerGroups: List<CityGroup> = emptyList(),
    val cacheCategories: List<CacheCategoryUi> = emptyList(),
    val showCacheManager: Boolean = false,
    val cacheStats: CacheStats = CacheStats(),
    val manualLatText: String = "",
    val manualLngText: String = "",
    val feedbackMessage: String? = null,
    val lastShownSettingsStatus: String = "",
)

sealed interface WhereBusAction {
    data object Initialize : WhereBusAction
    data object MarkOnboardingDone : WhereBusAction
    data object RequestLocationPermission : WhereBusAction
    data object OpenSearch : WhereBusAction
    data object CloseSearch : WhereBusAction
    data class ChangePage(val page: RootPage) : WhereBusAction
    data class SearchChanged(val query: String) : WhereBusAction
    data class OpenNearbyLine(val flatIndex: Int) : WhereBusAction
    data class OpenSearchLine(val index: Int) : WhereBusAction
    data object CloseDetail : WhereBusAction
    data object SwitchDetailDirection : WhereBusAction
    data object RefreshAll : WhereBusAction
    data class LocationModeChanged(val mode: LocationMode) : WhereBusAction
    data class ManualLatChanged(val value: String) : WhereBusAction
    data class ManualLngChanged(val value: String) : WhereBusAction
    data object SaveManualLocation : WhereBusAction
    data class ThemeModeChanged(val mode: ThemeMode) : WhereBusAction
    data object OpenCityPicker : WhereBusAction
    data object CloseCityPicker : WhereBusAction
    data class CityFilterChanged(val query: String) : WhereBusAction
    data class SelectService(val serviceId: String) : WhereBusAction
    data object OpenCacheManager : WhereBusAction
    data object CloseCacheManager : WhereBusAction
    data object ClearCache : WhereBusAction
    data class ClearCacheCategory(val category: CacheCategory) : WhereBusAction
    data object DismissFeedback : WhereBusAction
}

data class CityGroup(
    val region: String,
    val services: List<ServiceInfo>,
)

data class CacheCategoryUi(
    val category: CacheCategory,
    val label: String,
    val count: Int,
    val description: String,
)
