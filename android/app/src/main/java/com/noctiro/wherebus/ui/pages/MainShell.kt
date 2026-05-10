@file:OptIn(androidx.compose.material3.ExperimentalMaterial3Api::class)

package com.noctiro.wherebus.ui.pages

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxWithConstraints
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Place
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.Search
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.NavigationRail
import androidx.compose.material3.NavigationRailItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.material3.VerticalDivider
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import com.noctiro.wherebus.ui.RootPage
import com.noctiro.wherebus.ui.WhereBusAction
import com.noctiro.wherebus.ui.WhereBusUiState

@Composable
fun MainShell(
    state: WhereBusUiState,
    onAction: (WhereBusAction) -> Unit,
) {
    BoxWithConstraints(modifier = Modifier.fillMaxSize()) {
        val wide = maxWidth >= 900.dp
        if (wide) {
            Row(modifier = Modifier.fillMaxSize()) {
                AppRail(
                    state = state,
                    onAction = onAction,
                )
                VerticalDivider(color = MaterialTheme.colorScheme.outline.copy(alpha = 0.15f))
                Scaffold(
                    modifier = Modifier.weight(1f),
                    topBar = {
                        MainTopBar(
                            state = state,
                            onRefresh = { onAction(WhereBusAction.RefreshAll) },
                            onSearch = { onAction(WhereBusAction.OpenSearch) },
                        )
                    },
                ) { padding ->
                    Box(modifier = Modifier.fillMaxSize().padding(padding)) {
                        MainPageContent(
                            state = state,
                            onAction = onAction,
                        )
                    }
                }
            }
        } else {
            Scaffold(
                topBar = {
                    MainTopBar(
                        state = state,
                        onRefresh = { onAction(WhereBusAction.RefreshAll) },
                        onSearch = { onAction(WhereBusAction.OpenSearch) },
                    )
                },
                bottomBar = {
                    AppBottomBar(
                        state = state,
                        onAction = onAction,
                    )
                },
            ) { padding ->
                Box(modifier = Modifier.fillMaxSize().padding(padding)) {
                    MainPageContent(
                        state = state,
                        onAction = onAction,
                    )
                }
            }
        }
    }
}

@Composable
private fun AppRail(
    state: WhereBusUiState,
    onAction: (WhereBusAction) -> Unit,
) {
    NavigationRail(
        modifier = Modifier.width(88.dp).padding(vertical = 16.dp),
        containerColor = Color.Transparent,
    ) {
        NavigationRailItem(
            selected = state.currentPage == RootPage.Nearby,
            onClick = { onAction(WhereBusAction.ChangePage(RootPage.Nearby)) },
            icon = { Icon(Icons.Default.Place, contentDescription = null) },
            label = { Text(text = "附近") },
        )
        NavigationRailItem(
            selected = state.currentPage == RootPage.Settings,
            onClick = { onAction(WhereBusAction.ChangePage(RootPage.Settings)) },
            icon = { Icon(Icons.Default.Settings, contentDescription = null) },
            label = { Text(text = "设置") },
        )
    }
}

@Composable
private fun AppBottomBar(
    state: WhereBusUiState,
    onAction: (WhereBusAction) -> Unit,
) {
    NavigationBar {
        NavigationBarItem(
            selected = state.currentPage == RootPage.Nearby,
            onClick = { onAction(WhereBusAction.ChangePage(RootPage.Nearby)) },
            icon = { Icon(Icons.Default.Place, contentDescription = null) },
            label = { Text(text = "附近") },
        )
        NavigationBarItem(
            selected = state.currentPage == RootPage.Settings,
            onClick = { onAction(WhereBusAction.ChangePage(RootPage.Settings)) },
            icon = { Icon(Icons.Default.Settings, contentDescription = null) },
            label = { Text(text = "设置") },
        )
    }
}

@Composable
private fun MainTopBar(
    state: WhereBusUiState,
    onRefresh: () -> Unit,
    onSearch: () -> Unit,
) {
    TopAppBar(
        title = {
            Text(
                text = state.cityLabel.ifBlank { "线路数据" },
                style = MaterialTheme.typography.titleLarge,
            )
        },
        actions = {
            if (state.currentPage == RootPage.Nearby) {
                IconButton(onClick = onSearch) {
                    Icon(Icons.Default.Search, contentDescription = "搜索")
                }
            }
            IconButton(onClick = onRefresh) {
                Icon(Icons.Default.Refresh, contentDescription = "刷新")
            }
        },
        colors = TopAppBarDefaults.topAppBarColors(containerColor = Color.Transparent),
    )
}

@Composable
private fun MainPageContent(
    state: WhereBusUiState,
    onAction: (WhereBusAction) -> Unit,
) {
    when (state.currentPage) {
        RootPage.Nearby -> NearbyPage(state, onAction)
        RootPage.Settings -> SettingsPage(state, onAction)
    }
}
