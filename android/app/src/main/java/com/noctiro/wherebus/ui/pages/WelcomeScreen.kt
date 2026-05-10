@file:OptIn(
    androidx.compose.material3.ExperimentalMaterial3Api::class,
)

package com.noctiro.wherebus.ui.pages

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.DirectionsBus
import androidx.compose.material.icons.filled.LocationCity
import androidx.compose.material.icons.filled.MyLocation
import androidx.compose.material.icons.filled.Search
import androidx.compose.material3.Button
import androidx.compose.material3.CenterAlignedTopAppBar
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilledTonalButton
import androidx.compose.material3.Icon
import androidx.compose.material3.ListItem
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.ModalBottomSheet
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.SegmentedButton
import androidx.compose.material3.SingleChoiceSegmentedButtonRow
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.material3.rememberModalBottomSheetState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import com.noctiro.wherebus.domain.LocationMode
import com.noctiro.wherebus.ui.WhereBusAction
import com.noctiro.wherebus.ui.WhereBusUiState
import com.noctiro.wherebus.ui.components.EmptyStateCard
import com.noctiro.wherebus.ui.components.SectionCard
import kotlinx.coroutines.launch

@Composable
fun WelcomeScreen(
    state: WhereBusUiState,
    onAction: (WhereBusAction) -> Unit,
) {
    var showCitySheet by remember { mutableStateOf(false) }
    val sheetState = rememberModalBottomSheetState(skipPartiallyExpanded = true)
    val scope = rememberCoroutineScope()

    Scaffold(
        topBar = {
            CenterAlignedTopAppBar(
                title = {
                    Column(horizontalAlignment = Alignment.CenterHorizontally) {
                        Text(
                            text = "欢迎使用 WhereBus",
                            style = MaterialTheme.typography.titleLarge,
                        )
                        Text(
                            text = "完成以下设置即可开始",
                            style = MaterialTheme.typography.bodySmall,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                },
                colors = TopAppBarDefaults.centerAlignedTopAppBarColors(
                    containerColor = Color.Transparent,
                ),
            )
        },
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .verticalScroll(rememberScrollState())
                .padding(horizontal = 16.dp),
            verticalArrangement = Arrangement.spacedBy(20.dp),
        ) {
            Spacer(modifier = Modifier.height(16.dp))

            SectionCard(
                title = "选择数据源",
                icon = Icons.Default.LocationCity,
                subtitle = if (state.cityLabel.isNotBlank()) "当前: ${state.cityLabel}"
                else "选择你所在城市的公交数据源",
            ) {
                FilledTonalButton(
                    onClick = { showCitySheet = true },
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Icon(Icons.Default.LocationCity, contentDescription = null)
                    Spacer(modifier = Modifier.padding(start = 8.dp))
                    Text(if (state.cityLabel.isNotBlank()) "更换数据源" else "选择城市/数据源")
                }
            }

            SectionCard(
                title = "定位方式",
                icon = Icons.Default.MyLocation,
                subtitle = "用于查找附近站点和线路",
            ) {
                SingleChoiceSegmentedButtonRow(modifier = Modifier.fillMaxWidth()) {
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

                if (state.config.locationMode == LocationMode.Auto && !state.locationPermissionGranted) {
                    FilledTonalButton(
                        onClick = { onAction(WhereBusAction.RequestLocationPermission) },
                        modifier = Modifier.fillMaxWidth(),
                    ) {
                        Text("授予定位权限")
                    }
                }

                if (state.config.locationMode == LocationMode.Auto && state.locationPermissionGranted) {
                    Text(
                        text = "已授权定位",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.primary,
                    )
                }

                if (state.config.locationMode == LocationMode.Manual) {
                    OutlinedTextField(
                        value = state.manualLatText,
                        onValueChange = { onAction(WhereBusAction.ManualLatChanged(it)) },
                        modifier = Modifier.fillMaxWidth(),
                        label = { Text("纬度") },
                        singleLine = true,
                    )
                    OutlinedTextField(
                        value = state.manualLngText,
                        onValueChange = { onAction(WhereBusAction.ManualLngChanged(it)) },
                        modifier = Modifier.fillMaxWidth(),
                        label = { Text("经度") },
                        singleLine = true,
                    )
                    FilledTonalButton(
                        onClick = { onAction(WhereBusAction.SaveManualLocation) },
                        modifier = Modifier.fillMaxWidth(),
                        enabled = state.manualLatText.isNotBlank() && state.manualLngText.isNotBlank(),
                    ) {
                        Text("保存坐标")
                    }
                }
            }

            Spacer(modifier = Modifier.weight(1f))

            val locationReady = when (state.config.locationMode) {
                LocationMode.Auto -> state.locationPermissionGranted
                LocationMode.Manual -> state.manualLatText.isNotBlank() && state.manualLngText.isNotBlank()
            }

            Button(
                onClick = { onAction(WhereBusAction.MarkOnboardingDone) },
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(bottom = 32.dp),
                enabled = state.cityLabel.isNotBlank() && locationReady,
            ) {
                Icon(Icons.Default.DirectionsBus, contentDescription = null)
                Spacer(modifier = Modifier.padding(start = 8.dp))
                Text("开始使用")
            }
        }
    }

    if (showCitySheet) {
        ModalBottomSheet(
            onDismissRequest = { showCitySheet = false },
            sheetState = sheetState,
        ) {
            CityPickerSheetContent(
                state = state,
                onAction = { action ->
                    onAction(action)
                    if (action is WhereBusAction.SelectService) {
                        scope.launch {
                            sheetState.hide()
                            showCitySheet = false
                        }
                    }
                },
            )
        }
    }
}

@Composable
private fun CityPickerSheetContent(
    state: WhereBusUiState,
    onAction: (WhereBusAction) -> Unit,
) {
    Column(modifier = Modifier.fillMaxWidth()) {
        OutlinedTextField(
            value = state.cityPickerQuery,
            onValueChange = { onAction(WhereBusAction.CityFilterChanged(it)) },
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 16.dp)
                .padding(bottom = 8.dp),
            label = { Text("搜索城市或数据源") },
            singleLine = true,
            leadingIcon = { Icon(Icons.Default.Search, contentDescription = null) },
        )

        if (state.cityPickerGroups.isEmpty()) {
            EmptyStateCard(
                icon = Icons.Default.LocationCity,
                title = "没有匹配结果",
                message = "换一个关键词试试。",
            )
        } else {
            LazyColumn(
                modifier = Modifier.fillMaxWidth().height(400.dp),
                contentPadding = PaddingValues(bottom = 16.dp),
            ) {
                state.cityPickerGroups.forEach { group ->
                    stickyHeader(key = group.region) {
                        Text(
                            text = group.region,
                            style = MaterialTheme.typography.labelLarge,
                            color = MaterialTheme.colorScheme.primary,
                            modifier = Modifier
                                .fillMaxWidth()
                                .background(MaterialTheme.colorScheme.surfaceContainerLow)
                                .padding(horizontal = 16.dp, vertical = 8.dp),
                        )
                    }
                    items(group.services, key = { it.id }) { service ->
                        ListItem(
                            headlineContent = { Text(service.cityName) },
                            supportingContent = if (service.providerName.isNotBlank()) {
                                { Text(service.providerName) }
                            } else null,
                            modifier = Modifier.clickable {
                                onAction(WhereBusAction.SelectService(service.id))
                            },
                        )
                    }
                }
            }
        }
    }
}
