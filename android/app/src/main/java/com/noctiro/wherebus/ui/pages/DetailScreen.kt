@file:OptIn(ExperimentalMaterial3Api::class)

package com.noctiro.wherebus.ui.pages

import androidx.compose.animation.AnimatedContent
import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.animateDpAsState
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.spring
import androidx.compose.animation.core.tween
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.slideInVertically
import androidx.compose.animation.slideOutVertically
import androidx.compose.animation.togetherWith
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.IntrinsicSize
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.AltRoute
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.AltRoute
import androidx.compose.material.icons.filled.DirectionsBus
import androidx.compose.material.icons.filled.LocationOn
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.SignalWifiOff
import androidx.compose.material.icons.filled.SwapVert
import androidx.compose.material3.Badge
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FilledTonalIconButton
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.derivedStateOf
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.noctiro.wherebus.domain.BusItemUi
import com.noctiro.wherebus.domain.CongestionLevel
import com.noctiro.wherebus.domain.StationItemUi
import com.noctiro.wherebus.ui.WhereBusAction
import com.noctiro.wherebus.ui.WhereBusUiState
import com.noctiro.wherebus.ui.components.BusPill
import com.noctiro.wherebus.ui.components.EmptyStateCard
import com.noctiro.wherebus.ui.components.StopBadge
import kotlinx.coroutines.launch

// ─── 主屏 ────────────────────────────────────────────────────────

@Composable
fun DetailScreen(
    state: WhereBusUiState,
    onAction: (WhereBusAction) -> Unit,
    onNavigateBack: () -> Unit = {},
) {
    val snapshot = state.detail
    val stations = snapshot?.detail?.stations ?: emptyList()

    val nearestIndex by remember(stations) {
        derivedStateOf { stations.indexOfFirst { it.isNearest }.coerceAtLeast(0) }
    }

    var selectedIndex by remember(nearestIndex) { mutableIntStateOf(nearestIndex) }

    val listState = rememberLazyListState()
    val scope = rememberCoroutineScope()

    LaunchedEffect(nearestIndex, snapshot != null) {
        if (snapshot != null && nearestIndex > 2) {
            listState.animateScrollToItem(
                index = nearestIndex + 3,
                scrollOffset = -60,
            )
        }
    }

    Scaffold(
        topBar = {
            DetailTopBar(
                lineName = snapshot?.detail?.lineName ?: "线路详情",
                isLoading = state.detailLoading && snapshot != null,
                canSwitch = state.detailCanSwitch,
                onBack = onNavigateBack,
                onRefresh = { onAction(WhereBusAction.RefreshAll) },
                onSwitch = { onAction(WhereBusAction.SwitchDetailDirection) },
            )
        },
    ) { padding ->
        LazyColumn(
            state = listState,
            modifier = Modifier
                .fillMaxSize()
                .padding(padding),
            contentPadding = PaddingValues(bottom = 24.dp),
        ) {
            if (state.detailLoading && snapshot == null) {
                item {
                    Box(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(vertical = 80.dp),
                        contentAlignment = Alignment.Center,
                    ) {
                        Column(
                            horizontalAlignment = Alignment.CenterHorizontally,
                            verticalArrangement = Arrangement.spacedBy(16.dp),
                        ) {
                            CircularProgressIndicator(
                                modifier = Modifier.size(40.dp),
                                color = MaterialTheme.colorScheme.primary,
                            )
                            Text(
                                text = "正在加载线路详情…",
                                style = MaterialTheme.typography.bodyMedium,
                                color = MaterialTheme.colorScheme.onSurfaceVariant,
                            )
                        }
                    }
                }
            }

            snapshot?.let { snap ->
                val lineDetail = snap.detail

                item(key = "route_map") {
                    RouteMapCard(
                        stations = lineDetail.stations,
                        nearestIndex = nearestIndex,
                        selectedIndex = selectedIndex,
                        onSelectStation = { idx ->
                            selectedIndex = idx
                            scope.launch {
                                listState.animateScrollToItem(
                                    index = idx + 3,
                                    scrollOffset = -80,
                                )
                            }
                        },
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(horizontal = 16.dp)
                            .padding(top = 8.dp, bottom = 4.dp),
                    )
                }

                item(key = "eta_card") {
                    EtaCard(
                        stations = lineDetail.stations,
                        selectedIndex = selectedIndex,
                        nearestIndex = nearestIndex,
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(horizontal = 16.dp)
                            .padding(vertical = 8.dp),
                    )
                }

                item(key = "stops_label") {
                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(horizontal = 20.dp)
                            .padding(top = 8.dp, bottom = 6.dp),
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(6.dp),
                    ) {
                        Icon(
                            imageVector = Icons.AutoMirrored.Filled.AltRoute,
                            contentDescription = null,
                            modifier = Modifier.size(16.dp),
                            tint = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                        Text(
                            text = "全程站点 · ${lineDetail.stations.size} 站",
                            style = MaterialTheme.typography.labelMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                }

                val stationList = lineDetail.stations
                items(
                    count = stationList.size,
                    key = { i -> "station_$i" },
                ) { index ->
                    StationTimelineRow(
                        station = stationList[index],
                        displayOrder = stationList[index].order
                            .takeIf { it > 0 } ?: (index + 1),
                        isFirst = index == 0,
                        isLast = index == stationList.lastIndex,
                        isNearest = index == nearestIndex,
                        isSelected = index == selectedIndex,
                        isPassed = index < nearestIndex,
                        onSelect = {
                            selectedIndex = index
                        },
                        modifier = Modifier.padding(horizontal = 16.dp),
                    )
                }
            }

            if (state.detailError.isNotBlank()) {
                item(key = "error") {
                    Box(modifier = Modifier.padding(16.dp)) {
                        EmptyStateCard(
                            icon = Icons.Default.SignalWifiOff,
                            title = "详情加载失败",
                            message = state.detailError,
                            actionText = "重试",
                            onAction = { onAction(WhereBusAction.RefreshAll) },
                        )
                    }
                }
            }
        }
    }
}

// ─── TopAppBar ───────────────────────────────────────────────────

@Composable
private fun DetailTopBar(
    lineName: String,
    isLoading: Boolean,
    canSwitch: Boolean,
    onBack: () -> Unit,
    onRefresh: () -> Unit,
    onSwitch: () -> Unit,
) {
    Column {
        TopAppBar(
            title = {
                Text(
                    text = lineName,
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.Medium,
                )
            },
            navigationIcon = {
                IconButton(onClick = onBack) {
                    Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "返回")
                }
            },
            actions = {
                if (canSwitch) {
                    FilledTonalIconButton(onClick = onSwitch) {
                        Icon(Icons.Default.SwapVert, contentDescription = "切换方向")
                    }
                }
                IconButton(onClick = onRefresh) {
                    Icon(Icons.Default.Refresh, contentDescription = "刷新")
                }
            },
            colors = TopAppBarDefaults.topAppBarColors(
                containerColor = MaterialTheme.colorScheme.surface,
            ),
        )
        AnimatedVisibility(visible = isLoading) {
            LinearProgressIndicator(
                modifier = Modifier.fillMaxWidth(),
                color = MaterialTheme.colorScheme.primary,
                trackColor = MaterialTheme.colorScheme.primaryContainer,
            )
        }
    }
}

// ─── 线路图卡片 ──────────────────────────────────────────────────

@Composable
private fun RouteMapCard(
    stations: List<StationItemUi>,
    nearestIndex: Int,
    selectedIndex: Int,
    onSelectStation: (Int) -> Unit,
    modifier: Modifier = Modifier,
) {
    val primaryColor = MaterialTheme.colorScheme.primary
    val primaryContainerColor = MaterialTheme.colorScheme.primaryContainer
    val outlineVariantColor = MaterialTheme.colorScheme.outlineVariant
    val tertiaryColor = MaterialTheme.colorScheme.tertiary
    val onPrimaryColor = MaterialTheme.colorScheme.onPrimary
    val onSurfaceVariantColor = MaterialTheme.colorScheme.onSurfaceVariant

    val scrollState = rememberScrollState()

    LaunchedEffect(nearestIndex, stations.size) {
        if (stations.isNotEmpty()) {
            val targetScroll = ((nearestIndex - 2).coerceAtLeast(0) * 180)
            scrollState.animateScrollTo(targetScroll)
        }
    }

    Surface(
        modifier = modifier,
        shape = RoundedCornerShape(16.dp),
        color = MaterialTheme.colorScheme.surfaceContainerLow,
        tonalElevation = 1.dp,
    ) {
        Column(modifier = Modifier.padding(top = 14.dp, bottom = 10.dp)) {
            if (stations.isNotEmpty()) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp),
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.spacedBy(8.dp),
                ) {
                    Surface(
                        shape = CircleShape,
                        color = primaryContainerColor,
                        modifier = Modifier.size(8.dp),
                    ) {}
                    Text(
                        text = stations.first().name,
                        style = MaterialTheme.typography.labelSmall,
                        color = onSurfaceVariantColor,
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis,
                        modifier = Modifier.weight(1f),
                    )
                    Icon(
                        imageVector = Icons.AutoMirrored.Filled.AltRoute,
                        contentDescription = null,
                        modifier = Modifier.size(14.dp),
                        tint = onSurfaceVariantColor,
                    )
                    Text(
                        text = stations.last().name,
                        style = MaterialTheme.typography.labelSmall,
                        color = onSurfaceVariantColor,
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis,
                        modifier = Modifier.weight(1f),
                        textAlign = TextAlign.End,
                    )
                }
                Spacer(modifier = Modifier.height(10.dp))
            }

            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .horizontalScroll(scrollState)
                    .padding(horizontal = 12.dp),
                verticalAlignment = Alignment.Top,
            ) {
                stations.forEachIndexed { index, station ->
                    RouteMapStopItem(
                        name = station.name,
                        isNearest = index == nearestIndex,
                        isSelected = index == selectedIndex,
                        isPassed = index < nearestIndex,
                        hasBus = station.buses.isNotEmpty(),
                        isFirst = index == 0,
                        isLast = index == stations.lastIndex,
                        leftCongestion = station.prevCongestion,
                        rightCongestion = station.congestion,
                        primaryColor = primaryColor,
                        tertiaryColor = tertiaryColor,
                        outlineVariantColor = outlineVariantColor,
                        onPrimaryColor = onPrimaryColor,
                        onSurfaceVariantColor = onSurfaceVariantColor,
                        onClick = { onSelectStation(index) },
                    )
                }
            }
        }
    }
}

// ─── 线路图站点项 ────────────────────────────────────────────────

@Composable
private fun RouteMapStopItem(
    name: String,
    isNearest: Boolean,
    isSelected: Boolean,
    isPassed: Boolean,
    hasBus: Boolean,
    isFirst: Boolean,
    isLast: Boolean,
    leftCongestion: CongestionLevel?,
    rightCongestion: CongestionLevel?,
    primaryColor: Color,
    tertiaryColor: Color,
    outlineVariantColor: Color,
    onPrimaryColor: Color,
    onSurfaceVariantColor: Color,
    onClick: () -> Unit,
) {
    val dotSize by animateDpAsState(
        targetValue = when {
            isNearest -> 20.dp
            isSelected -> 16.dp
            hasBus -> 14.dp
            else -> 10.dp
        },
        animationSpec = spring(stiffness = Spring.StiffnessMedium),
        label = "dotSize",
    )
    val dotColor = when {
        hasBus -> tertiaryColor
        isPassed || isNearest || isSelected -> primaryColor
        else -> outlineVariantColor
    }

    val smoothColor = Color(0xFF4CAF50)
    val slowColor = Color(0xFFFFA726)
    val congestedColor = Color(0xFFEF5350)

    fun congestionColor(level: CongestionLevel?, fallback: Color): Color = when (level) {
        CongestionLevel.Smooth -> smoothColor
        CongestionLevel.Slow -> slowColor
        CongestionLevel.Congested -> congestedColor
        null -> fallback
    }

    val leftTrackColor = congestionColor(
        leftCongestion,
        if (isPassed || isNearest) primaryColor else outlineVariantColor,
    )
    val rightTrackColor = congestionColor(
        rightCongestion,
        if (isPassed) primaryColor else outlineVariantColor,
    )

    Column(
        modifier = Modifier
            .width(68.dp)
            .clickable(onClick = onClick),
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        // Fixed-height top area for bus badge (ensures track line aligns across all items)
        Box(
            modifier = Modifier
                .fillMaxWidth()
                .height(22.dp),
            contentAlignment = Alignment.BottomCenter,
        ) {
            if (hasBus) {
                Surface(
                    color = tertiaryColor,
                    shape = RoundedCornerShape(6.dp),
                ) {
                    Row(
                        modifier = Modifier.padding(horizontal = 6.dp, vertical = 2.dp),
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(2.dp),
                    ) {
                        Icon(
                            imageVector = Icons.Default.DirectionsBus,
                            contentDescription = null,
                            modifier = Modifier.size(10.dp),
                            tint = onPrimaryColor,
                        )
                        Text(
                            text = "在站",
                            style = MaterialTheme.typography.labelSmall,
                            fontSize = 10.sp,
                            color = onPrimaryColor,
                        )
                    }
                }
            }
        }

        Spacer(modifier = Modifier.height(3.dp))

        Box(
            modifier = Modifier
                .fillMaxWidth()
                .height(28.dp),
            contentAlignment = Alignment.Center,
        ) {
            // Full-width track line behind the dot
            Row(modifier = Modifier.fillMaxWidth().height(3.dp)) {
                if (!isFirst) {
                    Box(
                        modifier = Modifier
                            .weight(1f)
                            .fillMaxHeight()
                            .background(leftTrackColor),
                    )
                } else {
                    Spacer(modifier = Modifier.weight(1f))
                }
                if (!isLast) {
                    Box(
                        modifier = Modifier
                            .weight(1f)
                            .fillMaxHeight()
                            .background(rightTrackColor),
                    )
                } else {
                    Spacer(modifier = Modifier.weight(1f))
                }
            }
            Box(
                modifier = Modifier.size(28.dp),
                contentAlignment = Alignment.Center,
            ) {
                if (isNearest) {
                    Box(
                        modifier = Modifier
                            .size(28.dp)
                            .clip(CircleShape)
                            .background(primaryColor.copy(alpha = 0.2f)),
                    )
                }
                Box(
                    modifier = Modifier
                        .size(dotSize)
                        .clip(CircleShape)
                        .background(dotColor),
                ) {
                    if (isNearest) {
                        Icon(
                            imageVector = Icons.Default.LocationOn,
                            contentDescription = "你在这里",
                            modifier = Modifier
                                .size(14.dp)
                                .align(Alignment.Center),
                            tint = onPrimaryColor,
                        )
                    }
                }
            }
        }

        Spacer(modifier = Modifier.height(4.dp))

        Text(
            text = name,
            style = MaterialTheme.typography.labelSmall,
            fontSize = 10.sp,
            maxLines = 2,
            overflow = TextOverflow.Ellipsis,
            textAlign = TextAlign.Center,
            color = when {
                isNearest || isSelected -> primaryColor
                else -> onSurfaceVariantColor
            },
            fontWeight = if (isNearest || isSelected) FontWeight.Medium else FontWeight.Normal,
            lineHeight = 12.sp,
            modifier = Modifier.widthIn(max = 60.dp),
        )
    }
}

// ─── ETA 卡片 ────────────────────────────────────────────────────

@Composable
private fun EtaCard(
    stations: List<StationItemUi>,
    selectedIndex: Int,
    nearestIndex: Int,
    modifier: Modifier = Modifier,
) {
    val isNearestSelected = selectedIndex == nearestIndex
    val selectedStation = stations.getOrNull(selectedIndex)

    val arrivingBuses: List<Pair<BusItemUi, Int>> = remember(stations, selectedIndex) {
        stations.flatMapIndexed { stIdx, station ->
            if (stIdx > selectedIndex) return@flatMapIndexed emptyList()
            station.buses.map { bus ->
                val transitTime = (stIdx until selectedIndex).sumOf { i ->
                    stations[i].segmentTimeSeconds
                }
                val totalEtaSecs = bus.travelTimeSeconds.coerceAtLeast(0) + transitTime
                bus to (totalEtaSecs / 60)
            }
        }.sortedBy { it.second }
    }

    val nearestBusEta = arrivingBuses.firstOrNull()?.second

    val targetColor = if (isNearestSelected)
        MaterialTheme.colorScheme.primaryContainer
    else
        MaterialTheme.colorScheme.secondaryContainer

    val targetOnColor = if (isNearestSelected)
        MaterialTheme.colorScheme.onPrimaryContainer
    else
        MaterialTheme.colorScheme.onSecondaryContainer

    AnimatedContent(
        targetState = selectedIndex,
        transitionSpec = {
            (fadeIn(tween(200)) + slideInVertically { -it / 4 })
                .togetherWith(fadeOut(tween(150)) + slideOutVertically { it / 4 })
        },
        label = "etaCard",
        modifier = modifier,
    ) { _ ->
        Surface(
            shape = RoundedCornerShape(20.dp),
            color = targetColor,
            tonalElevation = 0.dp,
        ) {
            Column(modifier = Modifier.padding(16.dp)) {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    verticalAlignment = Alignment.Top,
                    horizontalArrangement = Arrangement.SpaceBetween,
                ) {
                    Column(modifier = Modifier.weight(1f)) {
                        Text(
                            text = if (isNearestSelected) "最近一班车到达"
                            else "到 ${selectedStation?.name ?: ""} 站",
                            style = MaterialTheme.typography.labelMedium,
                            color = targetOnColor.copy(alpha = 0.75f),
                        )
                        Spacer(modifier = Modifier.height(2.dp))
                        Text(
                            text = if (isNearestSelected)
                                "${selectedStation?.name ?: ""}（你在这里）"
                            else "从你的位置出发预计",
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.Medium,
                            color = targetOnColor,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis,
                        )
                    }

                    Spacer(modifier = Modifier.width(16.dp))

                    Column(horizontalAlignment = Alignment.End) {
                        if (nearestBusEta != null) {
                            Text(
                                text = nearestBusEta.toString(),
                                style = MaterialTheme.typography.displaySmall,
                                fontWeight = FontWeight.SemiBold,
                                color = targetOnColor,
                                lineHeight = 40.sp,
                            )
                            Text(
                                text = "分钟",
                                style = MaterialTheme.typography.labelMedium,
                                color = targetOnColor.copy(alpha = 0.75f),
                            )
                        } else {
                            Text(
                                text = "—",
                                style = MaterialTheme.typography.displaySmall,
                                color = targetOnColor.copy(alpha = 0.4f),
                            )
                            Text(
                                text = "暂无车辆",
                                style = MaterialTheme.typography.labelSmall,
                                color = targetOnColor.copy(alpha = 0.6f),
                            )
                        }
                    }
                }

                if (arrivingBuses.isNotEmpty()) {
                    Spacer(modifier = Modifier.height(12.dp))
                    Row(
                        horizontalArrangement = Arrangement.spacedBy(8.dp),
                        modifier = Modifier.horizontalScroll(rememberScrollState()),
                    ) {
                        arrivingBuses.take(3).forEach { (bus, eta) ->
                            BusPill(
                                bus = bus,
                                etaMinutes = eta,
                                isAtStation = bus.isArriving && eta == 0,
                                isHighlighted = isNearestSelected,
                            )
                        }
                    }
                }

                if (!isNearestSelected) {
                    Spacer(modifier = Modifier.height(6.dp))
                    Text(
                        text = "点击站点可切换 · 再次点击当前站返回",
                        style = MaterialTheme.typography.labelSmall,
                        color = targetOnColor.copy(alpha = 0.55f),
                    )
                }
            }
        }
    }
}

// ─── 站点时间轴行 ────────────────────────────────────────────────

@Composable
private fun StationTimelineRow(
    station: StationItemUi,
    displayOrder: Int,
    isFirst: Boolean,
    isLast: Boolean,
    isNearest: Boolean,
    isSelected: Boolean,
    isPassed: Boolean,
    onSelect: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val primaryColor = MaterialTheme.colorScheme.primary
    val outlineVariantColor = MaterialTheme.colorScheme.outlineVariant
    val tertiaryColor = MaterialTheme.colorScheme.tertiary

    val smoothColor = Color(0xFF4CAF50)
    val slowColor = Color(0xFFFFA726)
    val congestedColor = Color(0xFFEF5350)

    fun congestionColor(level: CongestionLevel?, fallback: Color): Color = when (level) {
        CongestionLevel.Smooth -> smoothColor
        CongestionLevel.Slow -> slowColor
        CongestionLevel.Congested -> congestedColor
        null -> fallback
    }

    val trackColor = congestionColor(
        station.prevCongestion,
        if (isPassed || isNearest) primaryColor else outlineVariantColor,
    )
    val bottomTrackColor = congestionColor(
        station.congestion,
        if (isPassed) primaryColor else outlineVariantColor,
    )
    val cardAlpha by animateFloatAsState(
        targetValue = if (isPassed && !isNearest) 0.55f else 1f,
        label = "cardAlpha",
    )

    val dotColor = when {
        station.buses.isNotEmpty() -> tertiaryColor
        isNearest || isSelected || isPassed -> primaryColor
        else -> outlineVariantColor
    }
    val dotSize by animateDpAsState(
        targetValue = when {
            isNearest -> 18.dp
            isSelected -> 14.dp
            station.buses.isNotEmpty() -> 14.dp
            else -> 10.dp
        },
        animationSpec = spring(stiffness = Spring.StiffnessMediumLow),
        label = "dotSize",
    )

    Row(
        modifier = modifier
            .fillMaxWidth()
            .height(IntrinsicSize.Min),
    ) {
        // 左侧：轨道 + 节点
        Box(
            modifier = Modifier
                .width(40.dp)
                .fillMaxHeight(),
        ) {
            // 上半段轨道（从顶部到节点中心）
            if (!isFirst) {
                Box(
                    modifier = Modifier
                        .width(2.dp)
                        .height(24.dp)
                        .align(Alignment.TopCenter)
                        .background(trackColor),
                )
            }
            // 下半段轨道（从节点中心到底部）
            if (!isLast) {
                Box(
                    modifier = Modifier
                        .width(2.dp)
                        .fillMaxHeight()
                        .padding(top = 24.dp)
                        .align(Alignment.TopCenter)
                        .background(bottomTrackColor),
                )
            }
            // 节点
            Box(
                modifier = Modifier
                    .padding(top = 12.dp)
                    .size(28.dp)
                    .align(Alignment.TopCenter),
                contentAlignment = Alignment.Center,
            ) {
                if (isNearest) {
                    Box(
                        modifier = Modifier
                            .size(24.dp)
                            .clip(CircleShape)
                            .background(primaryColor.copy(alpha = 0.2f)),
                    )
                }
                Box(
                    modifier = Modifier
                        .size(dotSize)
                        .clip(CircleShape)
                        .background(dotColor),
                    contentAlignment = Alignment.Center,
                ) {
                    if (isNearest) {
                        Icon(
                            imageVector = Icons.Default.LocationOn,
                            contentDescription = "你在这里",
                            modifier = Modifier.size(11.dp),
                            tint = MaterialTheme.colorScheme.onPrimary,
                        )
                    }
                }
            }
        }

        Spacer(modifier = Modifier.width(8.dp))

        // 右侧：卡片
        Column(
            modifier = Modifier
                .weight(1f)
                .alpha(cardAlpha)
                .padding(vertical = 3.dp),
        ) {
            Surface(
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable(onClick = onSelect),
                shape = RoundedCornerShape(12.dp),
                color = when {
                    isNearest -> MaterialTheme.colorScheme.primaryContainer
                    isSelected -> MaterialTheme.colorScheme.secondaryContainer
                    else -> MaterialTheme.colorScheme.surfaceContainerLow
                },
            ) {
                Column(modifier = Modifier.padding(12.dp)) {
                    Row(
                        verticalAlignment = Alignment.CenterVertically,
                        horizontalArrangement = Arrangement.spacedBy(8.dp),
                    ) {
                        Text(
                            text = displayOrder.toString().padStart(2, '0'),
                            style = MaterialTheme.typography.labelSmall,
                            color = when {
                                isNearest -> MaterialTheme.colorScheme.onPrimaryContainer.copy(alpha = 0.6f)
                                isSelected -> MaterialTheme.colorScheme.onSecondaryContainer.copy(alpha = 0.6f)
                                else -> MaterialTheme.colorScheme.onSurfaceVariant.copy(alpha = 0.5f)
                            },
                        )
                        Text(
                            text = station.name,
                            style = MaterialTheme.typography.bodyMedium,
                            fontWeight = if (isNearest || isSelected) FontWeight.Medium else FontWeight.Normal,
                            color = when {
                                isNearest -> MaterialTheme.colorScheme.onPrimaryContainer
                                isSelected -> MaterialTheme.colorScheme.onSecondaryContainer
                                else -> MaterialTheme.colorScheme.onSurface
                            },
                            modifier = Modifier.weight(1f),
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis,
                        )
                        if (isNearest) {
                            Badge(
                                containerColor = MaterialTheme.colorScheme.primary,
                                contentColor = MaterialTheme.colorScheme.onPrimary,
                            ) {
                                Text(
                                    text = "你在这里",
                                    style = MaterialTheme.typography.labelSmall,
                                )
                            }
                        } else if (isFirst) {
                            StopBadge("起点")
                        } else if (isLast) {
                            StopBadge("终点")
                        }
                    }

                    if (station.buses.isNotEmpty()) {
                        Spacer(modifier = Modifier.height(8.dp))
                        Row(
                            horizontalArrangement = Arrangement.spacedBy(6.dp),
                            modifier = Modifier.horizontalScroll(rememberScrollState()),
                        ) {
                            station.buses.forEach { bus ->
                                val eta = bus.travelTimeSeconds / 60
                                BusPill(
                                    bus = bus,
                                    etaMinutes = eta,
                                    isAtStation = bus.isArriving,
                                    isHighlighted = isNearest || isSelected,
                                )
                            }
                        }
                    } else if (isSelected && !isNearest) {
                        Spacer(modifier = Modifier.height(6.dp))
                        Text(
                            text = "已选中 · 在 ETA 卡片查看到达时间",
                            style = MaterialTheme.typography.labelSmall,
                            color = MaterialTheme.colorScheme.onSecondaryContainer.copy(alpha = 0.7f),
                        )
                    }
                }
            }
        }
    }
}
