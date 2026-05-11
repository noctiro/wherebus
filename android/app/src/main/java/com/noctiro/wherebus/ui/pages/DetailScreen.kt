package com.noctiro.wherebus.ui.pages

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.ModalBottomSheet
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.rememberModalBottomSheetState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.derivedStateOf
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableLongStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.noctiro.wherebus.domain.LineDetailUi
import com.noctiro.wherebus.domain.runStateLabel
import com.noctiro.wherebus.ui.WhereBusAction
import com.noctiro.wherebus.ui.WhereBusUiState
import com.noctiro.wherebus.ui.components.BusCardData
import com.noctiro.wherebus.ui.components.ErrorCard
import com.noctiro.wherebus.ui.components.computeBusCards
import kotlinx.coroutines.launch

// ─── Main Screen ────────────────────────────────────────────────

@OptIn(ExperimentalMaterial3Api::class)
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
    var selectedBusStationIndex by remember { mutableStateOf<Int?>(null) }
    var showInfoSheet by remember { mutableStateOf(false) }

    val listState = rememberLazyListState()
    val scope = rememberCoroutineScope()
    val density = LocalDensity.current

    val busCards = remember(stations, nearestIndex) {
        computeBusCards(stations, nearestIndex)
    }

    var lastUpdateTime by remember { mutableLongStateOf(System.currentTimeMillis()) }
    LaunchedEffect(snapshot) {
        if (snapshot != null) lastUpdateTime = System.currentTimeMillis()
    }

    LaunchedEffect(nearestIndex, snapshot != null) {
        if (snapshot != null && nearestIndex > 2) {
            listState.animateScrollToItem(
                index = nearestIndex,
                scrollOffset = with(density) { (-120).dp.roundToPx() },
            )
        }
    }

    Scaffold(
        topBar = {
            DetailFixedHeader(
                lineName = snapshot?.detail?.lineName ?: "线路详情",
                directionLabel = snapshot?.detail?.directionLabel ?: "",
                isLoading = state.detailLoading,
                hasData = snapshot != null,
                canSwitch = state.detailCanSwitch,
                lastUpdateTime = lastUpdateTime,
                busCards = busCards,
                selectedBusId = selectedBusStationIndex?.let { idx ->
                    busCards.find { it.currentStationIndex == idx }?.busId
                },
                onBack = onNavigateBack,
                onRefresh = { onAction(WhereBusAction.RefreshAll) },
                onSwitch = { onAction(WhereBusAction.SwitchDetailDirection) },
                onInfoClick = { showInfoSheet = true },
                onBusCardClick = { card ->
                    selectedBusStationIndex = card.currentStationIndex
                    selectedIndex = card.currentStationIndex
                    scope.launch {
                        listState.animateScrollToItem(
                            index = card.currentStationIndex,
                            scrollOffset = with(density) { (-120).dp.roundToPx() },
                        )
                    }
                },
            )
        },
    ) { padding ->
        if (state.detailLoading && snapshot == null) {
            Box(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(padding),
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
                        text = "加载中",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
        } else {
            LazyColumn(
                state = listState,
                modifier = Modifier
                    .fillMaxSize()
                    .padding(padding),
                contentPadding = PaddingValues(
                    top = 8.dp,
                    bottom = 24.dp,
                ),
            ) {
                snapshot?.let { snap ->
                    val stationList = snap.detail.stations
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
                            isSelected = index == selectedIndex && index != nearestIndex,
                            isPassed = index < nearestIndex,
                            nearestBusEta = busCards.firstOrNull()?.let { card ->
                                if (index == nearestIndex) card.etaMinutes else null
                            },
                            selectedBusEta = if (index == selectedIndex && index != nearestIndex) {
                                busCards.firstOrNull()?.let { card ->
                                    val transit = (card.currentStationIndex until index).sumOf { i ->
                                        stationList[i].segmentTimeSeconds
                                    }
                                    (card.etaMinutes * 60 + transit) / 60
                                }
                            } else null,
                            onSelect = {
                                selectedIndex = if (selectedIndex == index) nearestIndex else index
                                selectedBusStationIndex = null
                            },
                            modifier = Modifier.padding(horizontal = 16.dp),
                        )
                    }
                }

                if (state.detailError.isNotBlank()) {
                    item(key = "error") {
                        ErrorCard(
                            message = state.detailError,
                            onRetry = { onAction(WhereBusAction.RefreshAll) },
                            modifier = Modifier.padding(16.dp),
                        )
                    }
                }
            }
        }
    }

    if (showInfoSheet && snapshot != null) {
        LineInfoBottomSheet(
            detail = snapshot.detail,
            onDismiss = { showInfoSheet = false },
        )
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun LineInfoBottomSheet(
    detail: LineDetailUi,
    onDismiss: () -> Unit,
) {
    val sheetState = rememberModalBottomSheetState(skipPartiallyExpanded = true)

    ModalBottomSheet(
        onDismissRequest = onDismiss,
        sheetState = sheetState,
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 24.dp)
                .padding(bottom = 32.dp),
            verticalArrangement = Arrangement.spacedBy(4.dp),
        ) {
            // Title + run state
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    text = detail.lineName,
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.Medium,
                )
                detail.runState?.let { state ->
                    Text(
                        text = runStateLabel(state),
                        style = MaterialTheme.typography.labelMedium,
                        color = MaterialTheme.colorScheme.primary,
                    )
                }
            }

            if (detail.directionLabel.isNotBlank()) {
                Text(
                    text = detail.directionLabel,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }

            Spacer(modifier = Modifier.height(12.dp))
            HorizontalDivider()
            Spacer(modifier = Modifier.height(8.dp))

            // Schedule & price
            if (detail.firstTime.isNotBlank()) {
                InfoRow("首班", detail.firstTime)
            }
            if (detail.lastTime.isNotBlank()) {
                InfoRow("末班", detail.lastTime)
            }
            if (detail.planTime.isNotBlank()) {
                InfoRow("间隔", detail.planTime)
            }
            if (detail.price.isNotBlank()) {
                InfoRow("票价", detail.price)
            }

            // Operator info
            if (detail.company.isNotBlank() || detail.phone.isNotBlank()) {
                Spacer(modifier = Modifier.height(8.dp))
                HorizontalDivider()
                Spacer(modifier = Modifier.height(8.dp))

                if (detail.company.isNotBlank()) {
                    InfoRow("运营", detail.company)
                }
                if (detail.phone.isNotBlank()) {
                    InfoRow("电话", detail.phone)
                }
            }

            // Comments
            if (detail.comments.isNotBlank()) {
                Spacer(modifier = Modifier.height(8.dp))
                HorizontalDivider()
                Spacer(modifier = Modifier.height(8.dp))
                InfoRow("备注", detail.comments)
            }
        }
    }
}

@Composable
private fun InfoRow(label: String, value: String) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(vertical = 6.dp),
        horizontalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        Text(
            text = label,
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            modifier = Modifier.width(40.dp),
        )
        Text(
            text = value,
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.onSurface,
            fontWeight = FontWeight.Medium,
        )
    }
}
