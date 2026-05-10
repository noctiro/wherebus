@file:OptIn(androidx.compose.material3.ExperimentalMaterial3Api::class)

package com.noctiro.wherebus.ui.pages

import android.os.Build
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.GridItemSpan
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddLocationAlt
import androidx.compose.material.icons.filled.Cached
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.DirectionsBus
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.LocationCity
import androidx.compose.material.icons.filled.MyLocation
import androidx.compose.material.icons.filled.Palette
import androidx.compose.material.icons.filled.Route
import androidx.compose.material.icons.filled.Search
import androidx.compose.material.icons.filled.SignalWifiOff
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilledTonalButton
import androidx.compose.material3.Icon
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.ListItem
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.SegmentedButton
import androidx.compose.material3.SingleChoiceSegmentedButtonRow
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import com.noctiro.wherebus.BuildConfig
import com.noctiro.wherebus.domain.LocationMode
import com.noctiro.wherebus.domain.ThemeMode
import com.noctiro.wherebus.domain.label
import com.noctiro.wherebus.ui.RootPage
import com.noctiro.wherebus.ui.WhereBusAction
import com.noctiro.wherebus.ui.WhereBusUiState
import com.noctiro.wherebus.ui.components.EmptyStateCard
import com.noctiro.wherebus.ui.components.LineCard
import com.noctiro.wherebus.ui.components.SettingsGroupTitle
import com.noctiro.wherebus.ui.components.StationGroupCard

@Composable
fun NearbyPage(
    state: WhereBusUiState,
    onAction: (WhereBusAction) -> Unit,
) {
    LazyVerticalGrid(
        columns = GridCells.Adaptive(320.dp),
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(vertical = 12.dp),
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        if (state.nearbyLoading && state.nearbyGroups.isEmpty()) {
            item(span = { GridItemSpan(maxLineSpan) }) {
                Box(
                    modifier = Modifier.fillMaxWidth().padding(vertical = 48.dp),
                    contentAlignment = Alignment.Center,
                ) {
                    Column(horizontalAlignment = Alignment.CenterHorizontally, verticalArrangement = Arrangement.spacedBy(12.dp)) {
                        CircularProgressIndicator()
                        Text(text = "正在加载附近线路…", style = MaterialTheme.typography.bodyMedium, color = MaterialTheme.colorScheme.onSurfaceVariant)
                    }
                }
            }
        } else if (state.nearbyLoading) {
            item(span = { GridItemSpan(maxLineSpan) }) {
                LinearProgressIndicator(modifier = Modifier.fillMaxWidth())
            }
        }

        if (state.nearbyError.isNotBlank()) {
            item(span = { GridItemSpan(maxLineSpan) }) {
                EmptyStateCard(
                    icon = Icons.Default.SignalWifiOff,
                    title = "附近线路暂不可用",
                    message = state.nearbyError,
                    actionText = "重新加载",
                    onAction = { onAction(WhereBusAction.RefreshAll) },
                )
            }
        }

        if (state.nearbyGroups.isEmpty() && state.nearbyError.isBlank() && !state.nearbyLoading) {
            item(span = { GridItemSpan(maxLineSpan) }) {
                EmptyStateCard(
                    icon = Icons.Default.AddLocationAlt,
                    title = "还没有附近线路",
                    message = "去设置中补充位置，或者授权定位后再刷新。",
                    actionText = "去设置",
                    onAction = { onAction(WhereBusAction.ChangePage(RootPage.Settings)) },
                )
            }
        }

        items(state.nearbyGroups) { group ->
            StationGroupCard(
                group = group,
                onLineClick = { line -> onAction(WhereBusAction.OpenNearbyLine(line.flatIndex)) },
            )
        }
    }
}

@Composable
fun SettingsPage(
    state: WhereBusUiState,
    onAction: (WhereBusAction) -> Unit,
) {
    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(bottom = 16.dp),
    ) {
        // 数据源
        item { SettingsGroupTitle("数据源") }
        item {
            ListItem(
                headlineContent = { Text(state.cityLabel.ifBlank { "未选择城市" }) },
                supportingContent = { Text(state.providerName.ifBlank { "点击选择数据源" }) },
                leadingContent = { Icon(Icons.Default.LocationCity, contentDescription = null) },
                modifier = Modifier.clickable { onAction(WhereBusAction.OpenCityPicker) },
            )
        }

        // 定位
        item { SettingsGroupTitle("定位") }
        item {
            SingleChoiceSegmentedButtonRow(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 16.dp),
            ) {
                SegmentedButton(
                    selected = state.config.locationMode == LocationMode.Auto,
                    onClick = { onAction(WhereBusAction.LocationModeChanged(LocationMode.Auto)) },
                    shape = RoundedCornerShape(topStart = 12.dp, bottomStart = 12.dp),
                ) { Text("自动定位") }
                SegmentedButton(
                    selected = state.config.locationMode == LocationMode.Manual,
                    onClick = { onAction(WhereBusAction.LocationModeChanged(LocationMode.Manual)) },
                    shape = RoundedCornerShape(topEnd = 12.dp, bottomEnd = 12.dp),
                ) { Text("手动输入") }
            }
        }
        if (state.config.locationMode == LocationMode.Auto && !state.locationPermissionGranted) {
            item {
                ListItem(
                    headlineContent = { Text("授予定位权限") },
                    supportingContent = { Text("自动定位需要位置权限") },
                    leadingContent = { Icon(Icons.Default.MyLocation, contentDescription = null) },
                    trailingContent = {
                        FilledTonalButton(onClick = { onAction(WhereBusAction.RequestLocationPermission) }) {
                            Text("授权")
                        }
                    },
                )
            }
        }
        if (state.config.locationMode == LocationMode.Manual) {
            item {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp, vertical = 8.dp),
                    horizontalArrangement = Arrangement.spacedBy(12.dp),
                ) {
                    OutlinedTextField(
                        value = state.manualLatText,
                        onValueChange = { onAction(WhereBusAction.ManualLatChanged(it)) },
                        modifier = Modifier.weight(1f),
                        label = { Text("纬度") },
                        supportingText = { Text("-90 ~ 90") },
                        singleLine = true,
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
                    )
                    OutlinedTextField(
                        value = state.manualLngText,
                        onValueChange = { onAction(WhereBusAction.ManualLngChanged(it)) },
                        modifier = Modifier.weight(1f),
                        label = { Text("经度") },
                        supportingText = { Text("-180 ~ 180") },
                        singleLine = true,
                        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Decimal),
                    )
                }
            }
            item {
                Row(
                    modifier = Modifier.padding(horizontal = 16.dp),
                    horizontalArrangement = Arrangement.spacedBy(12.dp),
                ) {
                    Button(onClick = { onAction(WhereBusAction.SaveManualLocation) }) {
                        Text("保存并刷新")
                    }
                    FilledTonalButton(onClick = { onAction(WhereBusAction.RefreshAll) }) {
                        Text("仅刷新")
                    }
                }
            }
        }

        // 外观
        item { SettingsGroupTitle("外观") }
        item {
            SingleChoiceSegmentedButtonRow(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 16.dp),
            ) {
                ThemeMode.entries.forEachIndexed { index, mode ->
                    val first = index == 0
                    val last = index == ThemeMode.entries.lastIndex
                    SegmentedButton(
                        selected = state.config.themeMode == mode,
                        onClick = { onAction(WhereBusAction.ThemeModeChanged(mode)) },
                        shape = RoundedCornerShape(
                            topStart = if (first) 12.dp else 0.dp,
                            bottomStart = if (first) 12.dp else 0.dp,
                            topEnd = if (last) 12.dp else 0.dp,
                            bottomEnd = if (last) 12.dp else 0.dp,
                        ),
                    ) { Text(mode.label()) }
                }
            }
        }

        // 存储
        item { SettingsGroupTitle("存储") }
        item {
            ListItem(
                headlineContent = { Text("缓存管理") },
                supportingContent = { Text("已缓存 ${state.cacheStats.total()} 条数据") },
                leadingContent = { Icon(Icons.Default.Cached, contentDescription = null) },
                modifier = Modifier.clickable { onAction(WhereBusAction.OpenCacheManager) },
            )
        }
        item {
            ListItem(
                headlineContent = { Text("清空全部缓存") },
                supportingContent = { Text("清理后会自动重建") },
                leadingContent = { Icon(Icons.Default.Delete, contentDescription = null) },
                modifier = Modifier.clickable { onAction(WhereBusAction.ClearCache) },
            )
        }

        // 关于
        item { SettingsGroupTitle("关于") }
        item {
            ListItem(
                headlineContent = { Text("WhereBus") },
                supportingContent = {
                    Text("${BuildConfig.VERSION_NAME} · ${BuildConfig.BUILD_TYPE} · ${Build.SUPPORTED_ABIS.firstOrNull() ?: "-"}")
                },
                leadingContent = { Icon(Icons.Default.Info, contentDescription = null) },
            )
        }
    }
}
