package com.noctiro.wherebus.data

import android.content.Context
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json

internal object NativeWhereBusBridge {
    private val json = Json {
        ignoreUnknownKeys = true
        explicitNulls = false
    }

    @Volatile
    private var initialized = false

    fun initialize(context: Context) {
        if (initialized) {
            return
        }
        synchronized(this) {
            if (initialized) {
                return
            }
            System.loadLibrary("wherebus")
            setDataDir(context.filesDir.absolutePath)
            initialized = true
        }
    }

    fun bridgeUpdate(eventJson: String): String {
        val response = json.decodeFromString<NativeStringEnvelope>(update(eventJson))
        if (!response.ok) {
            throw IllegalStateException(response.error ?: "native bridge update failed")
        }
        return requireNotNull(response.data) { "native bridge update data missing" }
    }

    fun bridgeResolve(effectId: Int, responseJson: String): String {
        val response = json.decodeFromString<NativeStringEnvelope>(resolve(effectId, responseJson))
        if (!response.ok) {
            throw IllegalStateException(response.error ?: "native bridge resolve failed")
        }
        return requireNotNull(response.data) { "native bridge resolve data missing" }
    }

    fun bridgeView(): String {
        val response = json.decodeFromString<NativeStringEnvelope>(view())
        if (!response.ok) {
            throw IllegalStateException(response.error ?: "native bridge view failed")
        }
        return requireNotNull(response.data) { "native bridge view data missing" }
    }

    private external fun setDataDir(path: String)
    private external fun update(eventJson: String): String
    private external fun resolve(effectId: Int, responseJson: String): String
    private external fun view(): String
}

@Serializable
internal data class NativeStringEnvelope(
    val ok: Boolean,
    val data: String? = null,
    val error: String? = null,
)
