package com.noctiro.wherebus.ui.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.DirectionsBus
import androidx.compose.material.icons.filled.SignalWifiOff
import androidx.compose.material3.FilledTonalButton
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.noctiro.wherebus.domain.BusItemUi
import com.noctiro.wherebus.domain.StationItemUi
import com.noctiro.wherebus.domain.crowdLabel
import com.noctiro.wherebus.domain.formatDistanceCompact
import com.noctiro.wherebus.domain.formatDurationCompact

@Composable
fun BusPill(
    bus: BusItemUi,
    etaMinutes: Int,
    isAtStation: Boolean,
    isHighlighted: Boolean,
    modifier: Modifier = Modifier,
) {
    val bgColor = when {
        isAtStation -> MaterialTheme.colorScheme.tertiary
        isHighlighted -> MaterialTheme.colorScheme.primary.copy(alpha = 0.15f)
        else -> MaterialTheme.colorScheme.surfaceVariant
    }
    val textColor = when {
        isAtStation -> MaterialTheme.colorScheme.onTertiary
        isHighlighted -> MaterialTheme.colorScheme.primary
        else -> MaterialTheme.colorScheme.onSurfaceVariant
    }

    Surface(
        shape = RoundedCornerShape(50),
        color = bgColor,
        modifier = modifier,
    ) {
        Row(
            modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(4.dp),
        ) {
            Icon(
                imageVector = Icons.Default.DirectionsBus,
                contentDescription = null,
                modifier = Modifier.size(12.dp),
                tint = textColor,
            )
            val label = buildString {
                if (bus.busId.isNotBlank()) {
                    append(bus.busId)
                    append(" · ")
                }
                if (isAtStation) append("在站")
                else append("${etaMinutes}分")
            }
            Text(
                text = label,
                style = MaterialTheme.typography.labelSmall,
                fontWeight = if (isAtStation || isHighlighted) FontWeight.Medium else FontWeight.Normal,
                color = textColor,
            )
        }
    }
}

@Composable
fun StopBadge(label: String) {
    Surface(
        shape = RoundedCornerShape(50),
        color = MaterialTheme.colorScheme.surfaceVariant,
    ) {
        Text(
            text = label,
            style = MaterialTheme.typography.labelSmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
            modifier = Modifier.padding(horizontal = 8.dp, vertical = 2.dp),
        )
    }
}

// ─── Bus card model & computation ───────────────────────────────

data class BusCardData(
    val busId: String,
    val etaMinutes: Int,
    val currentStationName: String,
    val currentStationIndex: Int,
    val isAtStation: Boolean,
)

fun computeBusCards(
    stations: List<StationItemUi>,
    nearestIndex: Int,
): List<BusCardData> {
    val cards = mutableListOf<BusCardData>()
    for ((stIdx, station) in stations.withIndex()) {
        if (stIdx > nearestIndex) break
        for (bus in station.buses) {
            val transitTime = (stIdx until nearestIndex).sumOf { i ->
                stations[i].segmentTimeSeconds
            }
            val totalEtaSecs = bus.travelTimeSeconds.coerceAtLeast(0) + transitTime
            val etaMin = totalEtaSecs / 60
            cards.add(
                BusCardData(
                    busId = bus.busId,
                    etaMinutes = etaMin,
                    currentStationName = station.name,
                    currentStationIndex = stIdx,
                    isAtStation = bus.isArriving && etaMin <= 0,
                )
            )
        }
    }
    return cards.sortedWith(compareBy({ !it.isAtStation }, { it.etaMinutes }))
}

// ─── Error card ─────────────────────────────────────────────────

@Composable
fun ErrorCard(
    message: String,
    onRetry: () -> Unit,
    modifier: Modifier = Modifier,
) {
    Surface(
        modifier = modifier.fillMaxWidth(),
        shape = RoundedCornerShape(16.dp),
        color = MaterialTheme.colorScheme.errorContainer.copy(alpha = 0.3f),
    ) {
        Column(
            modifier = Modifier.padding(20.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Icon(
                imageVector = Icons.Default.SignalWifiOff,
                contentDescription = null,
                modifier = Modifier.size(32.dp),
                tint = MaterialTheme.colorScheme.error,
            )
            Text(
                text = message,
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurface,
            )
            FilledTonalButton(onClick = onRetry) {
                Text("重试")
            }
        }
    }
}
