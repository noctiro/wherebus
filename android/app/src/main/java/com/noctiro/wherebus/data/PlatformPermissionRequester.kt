package com.noctiro.wherebus.data

interface PlatformPermissionRequester {
    fun isLocationPermissionGranted(): Boolean

    suspend fun requestLocationPermission(): Boolean
}
