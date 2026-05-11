package com.noctiro.wherebus.ui

import android.content.Context
import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import androidx.lifecycle.viewModelScope
import com.noctiro.wherebus.data.AndroidLocationProvider
import com.noctiro.wherebus.data.CruxRuntime
import com.noctiro.wherebus.data.CruxUserEvent
import com.noctiro.wherebus.data.PlatformPermissionRequester
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.SharedFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asSharedFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

sealed interface NavigationEvent {
    data object ToDetail : NavigationEvent
    data object ToSearch : NavigationEvent
    data object ToCityPicker : NavigationEvent
    data object ToCacheManager : NavigationEvent
    data object PopDetail : NavigationEvent
    data object PopCityPicker : NavigationEvent
    data object PopCacheManager : NavigationEvent
}

class WhereBusViewModel(
    private val cruxRuntime: CruxRuntime,
) : ViewModel() {
    private val _uiState = MutableStateFlow(WhereBusUiState())
    val uiState: StateFlow<WhereBusUiState> = _uiState.asStateFlow()

    private val _navEvents = MutableSharedFlow<NavigationEvent>(extraBufferCapacity = 1)
    val navEvents: SharedFlow<NavigationEvent> = _navEvents.asSharedFlow()

    private var autoRefreshJob: Job? = null
    private var nearbyRefreshJob: Job? = null

    init {
        dispatch(WhereBusAction.Initialize)
    }

    fun dispatch(action: WhereBusAction) {
        when (action) {
            WhereBusAction.Initialize -> viewModelScope.launch {
                syncFromCore(cruxRuntime.start())
                startNearbyRefresh()
            }
            WhereBusAction.MarkOnboardingDone -> viewModelScope.launch {
                val state = _uiState.value
                if (state.config.locationMode == com.noctiro.wherebus.domain.LocationMode.Manual) {
                    syncFromCore(cruxRuntime.dispatch(
                        CruxUserEvent.SettingsSaveLocation(lat = state.manualLatText, lng = state.manualLngText),
                    ))
                }
                syncFromCore(cruxRuntime.dispatch(CruxUserEvent.WelcomeStart))
            }
            WhereBusAction.RequestLocationPermission -> emit(CruxUserEvent.WelcomeRequestLocation)
            WhereBusAction.OpenSearch -> {
                _uiState.update { it.copy(showSearch = true) }
                _navEvents.tryEmit(NavigationEvent.ToSearch)
            }
            WhereBusAction.CloseSearch -> _uiState.update { it.copy(showSearch = false) }
            is WhereBusAction.ChangePage -> {
                emit(
                    CruxUserEvent.ChangePage(
                        when (action.page) {
                            RootPage.Nearby -> 0
                            RootPage.Settings -> 1
                        },
                    ),
                )
            }
            is WhereBusAction.SearchChanged -> {
                _uiState.update { it.copy(searchQuery = action.query) }
                emit(CruxUserEvent.SearchChanged(action.query))
            }
            is WhereBusAction.OpenNearbyLine -> {
                viewModelScope.launch {
                    syncFromCore(cruxRuntime.dispatch(CruxUserEvent.NearbyLineSelected(action.flatIndex)))
                }
            }
            is WhereBusAction.OpenSearchLine -> {
                viewModelScope.launch {
                    syncFromCore(cruxRuntime.dispatch(CruxUserEvent.SearchLineSelected(action.index)))
                }
            }
            WhereBusAction.CloseDetail -> emit(CruxUserEvent.DetailGoBack)
            WhereBusAction.SwitchDetailDirection -> emit(CruxUserEvent.DetailSwitchDirection)
            WhereBusAction.RefreshAll -> refreshCurrentContext()
            is WhereBusAction.LocationModeChanged -> emit(
                CruxUserEvent.SettingsLocationModeChanged(action.mode == com.noctiro.wherebus.domain.LocationMode.Auto),
            )
            is WhereBusAction.ManualLatChanged -> {
                val filtered = action.value.filterCoordinate()
                _uiState.update { it.copy(manualLatText = filtered) }
            }
            is WhereBusAction.ManualLngChanged -> {
                val filtered = action.value.filterCoordinate()
                _uiState.update { it.copy(manualLngText = filtered) }
            }
            WhereBusAction.SaveManualLocation -> viewModelScope.launch {
                val state = _uiState.value
                emit(
                    CruxUserEvent.SettingsSaveLocation(
                        lat = state.manualLatText,
                        lng = state.manualLngText,
                    ),
                )
            }
            is WhereBusAction.ThemeModeChanged -> emit(
                CruxUserEvent.SettingsThemeChanged(
                    when (action.mode) {
                        com.noctiro.wherebus.domain.ThemeMode.System -> 0
                        com.noctiro.wherebus.domain.ThemeMode.Light -> 1
                        com.noctiro.wherebus.domain.ThemeMode.Dark -> 2
                    },
                ),
            )
            WhereBusAction.OpenCityPicker -> emit(CruxUserEvent.SettingsOpenCityPicker)
            WhereBusAction.CloseCityPicker -> emit(CruxUserEvent.CityPickerGoBack)
            is WhereBusAction.CityFilterChanged -> {
                _uiState.update { it.copy(cityPickerQuery = action.query) }
                emit(CruxUserEvent.CityPickerFilter(action.query))
            }
            is WhereBusAction.SelectService -> emit(CruxUserEvent.CityPickerSelected(action.serviceId))
            WhereBusAction.OpenCacheManager -> emit(CruxUserEvent.SettingsOpenCacheManager)
            WhereBusAction.CloseCacheManager -> emit(CruxUserEvent.CacheManagerGoBack)
            WhereBusAction.ClearCache -> emit(CruxUserEvent.CacheManagerClearAll)
            is WhereBusAction.ClearCacheCategory -> emit(
                CruxUserEvent.CacheManagerClearCategory(
                    when (action.category) {
                        com.noctiro.wherebus.domain.CacheCategory.Stations -> 0
                        com.noctiro.wherebus.domain.CacheCategory.StationLines -> 1
                        com.noctiro.wherebus.domain.CacheCategory.LineDetail -> 2
                        com.noctiro.wherebus.domain.CacheCategory.AllLines -> 3
                    },
                ),
            )
            WhereBusAction.DismissFeedback -> _uiState.update { it.copy(feedbackMessage = null) }
        }
    }

    private fun emit(event: CruxUserEvent) {
        viewModelScope.launch {
            syncFromCore(cruxRuntime.dispatch(event))
        }
    }

    private fun refreshCurrentContext() {
        viewModelScope.launch {
            val state = _uiState.value
            syncFromCore(cruxRuntime.dispatch(CruxUserEvent.NearbyRefresh))
            if (state.showDetail) {
                syncFromCore(cruxRuntime.dispatch(CruxUserEvent.DetailRefresh))
            }
        }
    }

    private fun startAutoRefresh() {
        if (autoRefreshJob?.isActive == true) return
        autoRefreshJob = viewModelScope.launch {
            while (true) {
                delay(AUTO_REFRESH_INTERVAL_MS)
                if (_uiState.value.showDetail) {
                    syncFromCore(cruxRuntime.dispatch(CruxUserEvent.DetailRefresh))
                } else {
                    break
                }
            }
        }
    }

    private fun stopAutoRefresh() {
        autoRefreshJob?.cancel()
        autoRefreshJob = null
    }

    private fun startNearbyRefresh() {
        if (nearbyRefreshJob?.isActive == true) return
        nearbyRefreshJob = viewModelScope.launch {
            while (true) {
                delay(NEARBY_REFRESH_INTERVAL_MS)
                if (!_uiState.value.showDetail) {
                    syncFromCore(cruxRuntime.dispatch(CruxUserEvent.NearbyRefresh))
                }
            }
        }
    }

    private fun stopNearbyRefresh() {
        nearbyRefreshJob?.cancel()
        nearbyRefreshJob = null
    }

    private fun syncFromCore(view: com.noctiro.wherebus.data.NativeAppView) {
        val previous = _uiState.value
        val newFeedback = if (view.settings_status.isNotBlank() && view.settings_status != previous.lastShownSettingsStatus) {
            view.settings_status
        } else {
            previous.feedbackMessage
        }
        val newState = view.toUiState(newFeedback).copy(
            showSearch = previous.showSearch,
            searchQuery = previous.searchQuery,
            cityPickerQuery = previous.cityPickerQuery,
            manualLatText = if (previous.initialSync) view.settings_lat else previous.manualLatText,
            manualLngText = if (previous.initialSync) view.settings_lng else previous.manualLngText,
            lastShownSettingsStatus = if (view.settings_status.isNotBlank()) view.settings_status else previous.lastShownSettingsStatus,
        )
        _uiState.value = newState

        if (newState.showDetail && !previous.showDetail) {
            _navEvents.tryEmit(NavigationEvent.ToDetail)
            startAutoRefresh()
        } else if (!newState.showDetail && previous.showDetail) {
            _navEvents.tryEmit(NavigationEvent.PopDetail)
            stopAutoRefresh()
        }

        if (newState.showCityPicker && !previous.showCityPicker) {
            _navEvents.tryEmit(NavigationEvent.ToCityPicker)
        } else if (!newState.showCityPicker && previous.showCityPicker) {
            _navEvents.tryEmit(NavigationEvent.PopCityPicker)
        }

        if (newState.showCacheManager && !previous.showCacheManager) {
            _navEvents.tryEmit(NavigationEvent.ToCacheManager)
        } else if (!newState.showCacheManager && previous.showCacheManager) {
            _navEvents.tryEmit(NavigationEvent.PopCacheManager)
        }
    }

    companion object {
        private const val AUTO_REFRESH_INTERVAL_MS = 10_000L
        private const val NEARBY_REFRESH_INTERVAL_MS = 30_000L

        fun factory(
            context: Context,
            permissionRequester: PlatformPermissionRequester,
            cruxRuntime: CruxRuntime = CruxRuntime(
                context = context,
                locationProvider = AndroidLocationProvider(context),
                permissionRequester = permissionRequester,
            ),
        ): ViewModelProvider.Factory =
            object : ViewModelProvider.Factory {
                @Suppress("UNCHECKED_CAST")
                override fun <T : ViewModel> create(modelClass: Class<T>): T {
                    return WhereBusViewModel(cruxRuntime) as T
                }
            }
    }
}

private fun String.filterCoordinate(): String {
    val sb = StringBuilder()
    var hasDot = false
    for ((i, c) in this.withIndex()) {
        when {
            c == '-' && i == 0 -> sb.append(c)
            c == '.' && !hasDot -> { sb.append(c); hasDot = true }
            c.isDigit() -> sb.append(c)
        }
    }
    return sb.toString()
}