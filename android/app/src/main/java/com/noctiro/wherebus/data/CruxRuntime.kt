package com.noctiro.wherebus.data

import android.content.Context
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.buildJsonArray
import kotlinx.serialization.json.buildJsonObject

class CruxRuntime(
    context: Context,
    private val locationProvider: PlatformLocationProvider,
    private val permissionRequester: PlatformPermissionRequester,
) {
    private val appContext = context.applicationContext
    private val bridgeMutex = Mutex()
    private val json = Json {
        ignoreUnknownKeys = true
        explicitNulls = false
    }

    init {
        NativeWhereBusBridge.initialize(appContext)
    }

    suspend fun start(): NativeAppView = withContext(Dispatchers.IO) {
        bridgeMutex.withLock {
            val requests = NativeWhereBusBridge.bridgeUpdate("\"Boot\"")
            drainPlatformEffects(requests)

            val permissionSync = NativeWhereBusBridge.bridgeUpdate(
                CruxUserEvent.LocationPermissionLoaded(
                    permissionRequester.isLocationPermissionGranted(),
                ).toWireJson(),
            )
            drainPlatformEffects(permissionSync)
            currentViewLocked()
        }
    }

    suspend fun dispatch(event: CruxUserEvent): NativeAppView = withContext(Dispatchers.IO) {
        bridgeMutex.withLock {
            val requests = NativeWhereBusBridge.bridgeUpdate(event.toWireJson())
            drainPlatformEffects(requests)
            currentViewLocked()
        }
    }

    suspend fun currentView(): NativeAppView = withContext(Dispatchers.IO) {
        bridgeMutex.withLock {
            currentViewLocked()
        }
    }

    private suspend fun drainPlatformEffects(requestsJson: String) {
        var pending = decodeRequests(requestsJson)
        while (pending.isNotEmpty()) {
            val nextBatch = mutableListOf<CruxEffectRequest>()
            pending.forEach { request ->
                val responseJson = executePlatformEffect(request.effect)
                val produced = NativeWhereBusBridge.bridgeResolve(request.id, responseJson)
                nextBatch += decodeRequests(produced)
            }
            pending = nextBatch
        }
    }

    private suspend fun executePlatformEffect(effect: JsonObject): String {
        val (kind, _) = effect.entries.single()
        return when (kind) {
            "RequestLocationPermission" -> JsonPrimitive(
                permissionRequester.requestLocationPermission(),
            ).toString()
            "FetchAutoLocation" -> resolveFetchAutoLocation()
            else -> error("Unsupported platform effect: $kind")
        }
    }

    private suspend fun resolveFetchAutoLocation(): String {
        val location = locationProvider.currentLocationOrNull().getOrNull()
        return if (location != null) {
            buildJsonObject {
                put("Ok", buildJsonArray {
                    add(JsonPrimitive(location.first))
                    add(JsonPrimitive(location.second))
                })
            }.toString()
        } else {
            buildJsonObject {
                put("Err", JsonPrimitive("定位失败"))
            }.toString()
        }
    }

    private fun decodeRequests(requestsJson: String): List<CruxEffectRequest> {
        if (requestsJson.isBlank()) {
            return emptyList()
        }
        return json.decodeFromString(requestsJson)
    }

    private fun currentViewLocked(): NativeAppView {
        return json.decodeFromString(NativeWhereBusBridge.bridgeView())
    }
}

@Serializable
private data class CruxEffectRequest(
    val id: Int,
    val effect: JsonObject,
)
