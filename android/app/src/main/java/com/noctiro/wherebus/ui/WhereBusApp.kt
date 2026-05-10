package com.noctiro.wherebus.ui

import androidx.compose.animation.AnimatedContentTransitionScope
import androidx.compose.animation.core.tween
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.SnackbarHost
import androidx.compose.material3.SnackbarHostState
import androidx.compose.material3.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.lifecycle.viewmodel.compose.viewModel
import androidx.navigation.NavController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.noctiro.wherebus.data.PlatformPermissionRequester
import com.noctiro.wherebus.ui.pages.CacheManagerScreen
import com.noctiro.wherebus.ui.pages.CityPickerScreen
import com.noctiro.wherebus.ui.pages.DetailScreen
import com.noctiro.wherebus.ui.pages.LoadingScreen
import com.noctiro.wherebus.ui.pages.MainShell
import com.noctiro.wherebus.ui.pages.SearchScreen
import com.noctiro.wherebus.ui.pages.WelcomeScreen
import com.noctiro.wherebus.ui.theme.WhereBusTheme
import kotlinx.serialization.Serializable

private const val DURATION = 350

@Serializable data object MainRoute
@Serializable data object SearchRoute
@Serializable data object DetailRoute
@Serializable data object CityPickerRoute
@Serializable data object CacheManagerRoute

@Composable
fun WhereBusApp(
    permissionRequester: PlatformPermissionRequester,
    viewModel: WhereBusViewModel = viewModel(
        factory = WhereBusViewModel.factory(
            context = LocalContext.current,
            permissionRequester = permissionRequester,
        ),
    ),
) {
    val uiState by viewModel.uiState.collectAsState()
    val snackbarHostState = remember { SnackbarHostState() }
    val navController = rememberNavController()

    LaunchedEffect(uiState.feedbackMessage) {
        val message = uiState.feedbackMessage ?: return@LaunchedEffect
        snackbarHostState.showSnackbar(message)
        viewModel.dispatch(WhereBusAction.DismissFeedback)
    }

    LaunchedEffect(Unit) {
        viewModel.navEvents.collect { event ->
            when (event) {
                NavigationEvent.ToDetail -> navController.navigateSingleTop(DetailRoute)
                NavigationEvent.ToSearch -> navController.navigateSingleTop(SearchRoute)
                NavigationEvent.ToCityPicker -> navController.navigateSingleTop(CityPickerRoute)
                NavigationEvent.ToCacheManager -> navController.navigateSingleTop(CacheManagerRoute)
                NavigationEvent.PopDetail -> navController.popIfCurrent<DetailRoute>()
                NavigationEvent.PopCityPicker -> navController.popIfCurrent<CityPickerRoute>()
                NavigationEvent.PopCacheManager -> navController.popIfCurrent<CacheManagerRoute>()
            }
        }
    }

    WhereBusTheme(themeMode = uiState.config.themeMode) {
        Surface(
            modifier = Modifier.fillMaxSize(),
            color = MaterialTheme.colorScheme.background,
        ) {
            Box(
                modifier = Modifier
                    .fillMaxSize()
                    .background(
                        Brush.verticalGradient(
                            listOf(
                                MaterialTheme.colorScheme.primary.copy(alpha = 0.15f),
                                MaterialTheme.colorScheme.background,
                            ),
                        ),
                    ),
            ) {
                when {
                    uiState.loading -> LoadingScreen()
                    uiState.showWelcome -> WelcomeScreen(
                        state = uiState,
                        onAction = viewModel::dispatch,
                    )
                    else -> {
                        NavHost(
                            navController = navController,
                            startDestination = MainRoute,
                            enterTransition = {
                                slideIntoContainer(
                                    AnimatedContentTransitionScope.SlideDirection.Left,
                                    tween(DURATION),
                                ) + fadeIn(tween(DURATION))
                            },
                            exitTransition = {
                                slideOutOfContainer(
                                    AnimatedContentTransitionScope.SlideDirection.Left,
                                    tween(DURATION),
                                ) + fadeOut(tween(DURATION))
                            },
                            popEnterTransition = {
                                slideIntoContainer(
                                    AnimatedContentTransitionScope.SlideDirection.Right,
                                    tween(DURATION),
                                ) + fadeIn(tween(DURATION))
                            },
                            popExitTransition = {
                                slideOutOfContainer(
                                    AnimatedContentTransitionScope.SlideDirection.Right,
                                    tween(DURATION),
                                ) + fadeOut(tween(DURATION))
                            },
                        ) {
                            composable<MainRoute> {
                                MainShell(
                                    state = uiState,
                                    onAction = viewModel::dispatch,
                                )
                            }
                            composable<SearchRoute> {
                                DisposableEffect(Unit) {
                                    onDispose { viewModel.dispatch(WhereBusAction.CloseSearch) }
                                }
                                SearchScreen(
                                    state = uiState,
                                    onAction = viewModel::dispatch,
                                    onNavigateBack = { navController.popBackStack() },
                                )
                            }
                            composable<DetailRoute> {
                                DisposableEffect(Unit) {
                                    onDispose { viewModel.dispatch(WhereBusAction.CloseDetail) }
                                }
                                DetailScreen(
                                    state = uiState,
                                    onAction = viewModel::dispatch,
                                    onNavigateBack = { navController.popBackStack() },
                                )
                            }
                            composable<CityPickerRoute> {
                                DisposableEffect(Unit) {
                                    onDispose { viewModel.dispatch(WhereBusAction.CloseCityPicker) }
                                }
                                CityPickerScreen(
                                    state = uiState,
                                    onAction = viewModel::dispatch,
                                    onNavigateBack = { navController.popBackStack() },
                                )
                            }
                            composable<CacheManagerRoute> {
                                DisposableEffect(Unit) {
                                    onDispose { viewModel.dispatch(WhereBusAction.CloseCacheManager) }
                                }
                                CacheManagerScreen(
                                    state = uiState,
                                    onAction = viewModel::dispatch,
                                    onNavigateBack = { navController.popBackStack() },
                                )
                            }
                        }
                    }
                }

                SnackbarHost(
                    hostState = snackbarHostState,
                    modifier = Modifier
                        .align(Alignment.BottomCenter)
                        .padding(16.dp),
                )
            }
        }
    }
}

private fun <T : Any> NavController.navigateSingleTop(route: T) {
    navigate(route) { launchSingleTop = true }
}

private inline fun <reified T : Any> NavController.popIfCurrent() {
    if (currentBackStackEntry?.destination?.route?.contains(T::class.qualifiedName ?: "") == true) {
        popBackStack()
    }
}
