package com.noctiro.wherebus.data

import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonObject

sealed interface CruxUserEvent {
    fun toWireJson(): String

    data class ChangePage(val page: Int) : CruxUserEvent {
        override fun toWireJson(): String = buildJsonObject {
            put("ChangePage", JsonPrimitive(page))
        }.toString()
    }

    data object WelcomeStart : CruxUserEvent {
        override fun toWireJson(): String = "\"WelcomeStart\""
    }

    data object WelcomeRequestLocation : CruxUserEvent {
        override fun toWireJson(): String = "\"WelcomeRequestLocation\""
    }

    data class LocationPermissionLoaded(val granted: Boolean) : CruxUserEvent {
        override fun toWireJson(): String = buildJsonObject {
            put("LocationPermissionLoaded", JsonPrimitive(granted))
        }.toString()
    }

    data object NearbyRefresh : CruxUserEvent {
        override fun toWireJson(): String = "\"NearbyRefresh\""
    }

    data class NearbyLineSelected(val index: Int) : CruxUserEvent {
        override fun toWireJson(): String = buildJsonObject {
            put("NearbyLineSelected", JsonPrimitive(index))
        }.toString()
    }

    data class SearchChanged(val query: String) : CruxUserEvent {
        override fun toWireJson(): String = buildJsonObject {
            put("SearchChanged", JsonPrimitive(query))
        }.toString()
    }

    data class SearchLineSelected(val index: Int) : CruxUserEvent {
        override fun toWireJson(): String = buildJsonObject {
            put("SearchLineSelected", JsonPrimitive(index))
        }.toString()
    }

    data object DetailGoBack : CruxUserEvent {
        override fun toWireJson(): String = "\"DetailGoBack\""
    }

    data object DetailSwitchDirection : CruxUserEvent {
        override fun toWireJson(): String = "\"DetailSwitchDirection\""
    }

    data object DetailRefresh : CruxUserEvent {
        override fun toWireJson(): String = "\"DetailRefresh\""
    }

    data class SettingsSaveLocation(val lat: String, val lng: String) : CruxUserEvent {
        override fun toWireJson(): String = buildJsonObject {
            put(
                "SettingsSaveLocation",
                kotlinx.serialization.json.JsonArray(listOf(JsonPrimitive(lat), JsonPrimitive(lng))),
            )
        }.toString()
    }

    data class SettingsLocationModeChanged(val isAuto: Boolean) : CruxUserEvent {
        override fun toWireJson(): String = buildJsonObject {
            put("SettingsLocationModeChanged", JsonPrimitive(isAuto))
        }.toString()
    }

    data object SettingsOpenCityPicker : CruxUserEvent {
        override fun toWireJson(): String = "\"SettingsOpenCityPicker\""
    }

    data object CityPickerGoBack : CruxUserEvent {
        override fun toWireJson(): String = "\"CityPickerGoBack\""
    }

    data class CityPickerSelected(val serviceId: String) : CruxUserEvent {
        override fun toWireJson(): String = buildJsonObject {
            put("CityPickerSelected", JsonPrimitive(serviceId))
        }.toString()
    }

    data class CityPickerFilter(val query: String) : CruxUserEvent {
        override fun toWireJson(): String = buildJsonObject {
            put("CityPickerFilter", JsonPrimitive(query))
        }.toString()
    }

    data class SettingsThemeChanged(val index: Int) : CruxUserEvent {
        override fun toWireJson(): String = buildJsonObject {
            put("SettingsThemeChanged", JsonPrimitive(index))
        }.toString()
    }

    data object SettingsClearCache : CruxUserEvent {
        override fun toWireJson(): String = "\"SettingsClearCache\""
    }

    data object SettingsOpenCacheManager : CruxUserEvent {
        override fun toWireJson(): String = "\"SettingsOpenCacheManager\""
    }

    data object CacheManagerGoBack : CruxUserEvent {
        override fun toWireJson(): String = "\"CacheManagerGoBack\""
    }

    data class CacheManagerClearCategory(val index: Int) : CruxUserEvent {
        override fun toWireJson(): String = buildJsonObject {
            put("CacheManagerClearCategory", JsonPrimitive(index))
        }.toString()
    }

    data object CacheManagerClearAll : CruxUserEvent {
        override fun toWireJson(): String = "\"CacheManagerClearAll\""
    }
}
