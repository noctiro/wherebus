package com.noctiro.wherebus.data

import android.annotation.SuppressLint
import android.content.Context
import android.location.Location
import android.location.LocationManager
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

interface PlatformLocationProvider {
    suspend fun currentLocationOrNull(): Result<Pair<Double, Double>?>
}

class AndroidLocationProvider(
    private val context: Context,
) : PlatformLocationProvider {
    @SuppressLint("MissingPermission")
    override suspend fun currentLocationOrNull(): Result<Pair<Double, Double>?> = withContext(Dispatchers.IO) {
        runCatching {
            val manager = context.getSystemService(LocationManager::class.java)
                ?: return@runCatching null
            val providers = buildList {
                add(LocationManager.GPS_PROVIDER)
                add(LocationManager.NETWORK_PROVIDER)
                add(LocationManager.PASSIVE_PROVIDER)
            }
            val best: Location? = providers
                .filter(manager::isProviderEnabled)
                .mapNotNull { provider ->
                    runCatching { manager.getLastKnownLocation(provider) }.getOrNull()
                }
                .maxByOrNull(Location::getTime)
            best?.let { it.latitude to it.longitude }
        }
    }
}

