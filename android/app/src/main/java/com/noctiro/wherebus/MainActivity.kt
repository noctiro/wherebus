package com.noctiro.wherebus

import android.Manifest
import android.content.pm.PackageManager
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.activity.result.contract.ActivityResultContracts
import androidx.core.content.ContextCompat
import com.noctiro.wherebus.data.PlatformPermissionRequester
import com.noctiro.wherebus.ui.WhereBusApp
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.suspendCancellableCoroutine
import kotlinx.coroutines.withContext
import kotlin.coroutines.resume

class MainActivity : ComponentActivity() {
    private var permissionResult: ((Boolean) -> Unit)? = null

    private val locationPermissionLauncher =
        registerForActivityResult(ActivityResultContracts.RequestMultiplePermissions()) { grants ->
            val granted =
                grants[Manifest.permission.ACCESS_FINE_LOCATION] == true ||
                    grants[Manifest.permission.ACCESS_COARSE_LOCATION] == true
            permissionResult?.invoke(granted)
            permissionResult = null
        }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        val permissionRequester =
            object : PlatformPermissionRequester {
                override fun isLocationPermissionGranted(): Boolean {
                    return hasLocationPermission()
                }

                override suspend fun requestLocationPermission(): Boolean {
                    return withContext(Dispatchers.Main.immediate) {
                        if (hasLocationPermission()) {
                            return@withContext true
                        }
                        suspendCancellableCoroutine { continuation ->
                            permissionResult = { granted ->
                                if (continuation.isActive) {
                                    continuation.resume(granted)
                                }
                            }
                            continuation.invokeOnCancellation {
                                permissionResult = null
                            }
                            locationPermissionLauncher.launch(
                                arrayOf(
                                    Manifest.permission.ACCESS_FINE_LOCATION,
                                    Manifest.permission.ACCESS_COARSE_LOCATION,
                                ),
                            )
                        }
                    }
                }
            }
        setContent {
            WhereBusApp(permissionRequester = permissionRequester)
        }
    }

    private fun hasLocationPermission(): Boolean {
        return ContextCompat.checkSelfPermission(
            this,
            Manifest.permission.ACCESS_FINE_LOCATION,
        ) == PackageManager.PERMISSION_GRANTED ||
            ContextCompat.checkSelfPermission(
                this,
                Manifest.permission.ACCESS_COARSE_LOCATION,
            ) == PackageManager.PERMISSION_GRANTED
    }
}
