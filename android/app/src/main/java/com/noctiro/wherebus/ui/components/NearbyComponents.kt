package com.noctiro.wherebus.ui.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Place
import androidx.compose.material3.Card
import androidx.compose.material3.CardDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.noctiro.wherebus.domain.NearbyGroup
import com.noctiro.wherebus.domain.NearbyLineView
import com.noctiro.wherebus.domain.ArrivalState
import com.noctiro.wherebus.domain.formatDistanceCompact
import com.noctiro.wherebus.domain.runStateLabel

@Composable
fun StationGroupCard(
    group: NearbyGroup,
    onLineClick: (NearbyLineView) -> Unit,
) {
    SectionCard(
        title = group.station.name,
        icon = Icons.Default.Place,
        subtitle = "距你 ${group.station.distanceMeters.formatDistanceCompact()} · ${group.station.lineCount} 条线路",
    ) {
        Column(verticalArrangement = Arrangement.spacedBy(8.dp)) {
            group.lines.forEach { line ->
                LineRow(line = line, onClick = { onLineClick(line) })
            }
        }
    }
}

@Composable
fun LineRow(
    line: NearbyLineView,
    onClick: () -> Unit,
) {
    Card(
        onClick = onClick,
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surfaceContainerHigh,
        ),
    ) {
        Row(
            modifier = Modifier.padding(horizontal = 16.dp, vertical = 12.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Column(modifier = Modifier.weight(1f), verticalArrangement = Arrangement.spacedBy(2.dp)) {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    horizontalArrangement = Arrangement.spacedBy(6.dp),
                ) {
                    Text(
                        text = line.lineName,
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.Bold,
                        color = MaterialTheme.colorScheme.onSurface,
                    )
                    if (line.runState != null) {
                        Text(
                            text = runStateLabel(line.runState),
                            style = MaterialTheme.typography.labelSmall.copy(
                                fontSize = MaterialTheme.typography.labelSmall.fontSize * 0.85f,
                            ),
                            color = MaterialTheme.colorScheme.outlineVariant,
                        )
                    }
                }
                if (line.origin.isNotBlank() && line.terminus.isNotBlank()) {
                    val originText = formatStationName(line.origin, line.originAlias)
                    val terminusText = formatStationName(line.terminus, line.terminusAlias)
                    Text(
                        text = "$originText → $terminusText",
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        maxLines = 1,
                        overflow = TextOverflow.Ellipsis,
                    )
                }
            }

            Column(
                horizontalAlignment = Alignment.End,
                verticalArrangement = Arrangement.spacedBy(2.dp),
                modifier = Modifier.padding(start = 12.dp),
            ) {
                when (line.arrivalState) {
                    ArrivalState.Arriving -> {
                        Text(
                            text = "即将到站",
                            style = MaterialTheme.typography.labelLarge,
                            fontWeight = FontWeight.Bold,
                            color = MaterialTheme.colorScheme.error,
                        )
                    }
                    ArrivalState.Approaching -> {
                        if (line.minutesAway > 0) {
                            Row(verticalAlignment = Alignment.Bottom) {
                                Text(
                                    text = "${line.minutesAway}",
                                    style = MaterialTheme.typography.headlineSmall,
                                    fontWeight = FontWeight.Bold,
                                    color = MaterialTheme.colorScheme.primary,
                                )
                                Text(
                                    text = "分钟",
                                    style = MaterialTheme.typography.labelMedium,
                                    color = MaterialTheme.colorScheme.primary,
                                    modifier = Modifier.padding(bottom = 2.dp, start = 2.dp),
                                )
                            }
                            Row(
                                horizontalArrangement = Arrangement.spacedBy(4.dp),
                                verticalAlignment = Alignment.CenterVertically,
                            ) {
                                if (line.stationsAway > 0) {
                                    Text(
                                        text = "${line.stationsAway}站",
                                        style = MaterialTheme.typography.labelSmall,
                                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                                    )
                                }
                                if (line.distanceMeters > 0) {
                                    if (line.stationsAway > 0) {
                                        Text(
                                            text = "/",
                                            style = MaterialTheme.typography.labelSmall,
                                            color = MaterialTheme.colorScheme.outlineVariant,
                                        )
                                    }
                                    Text(
                                        text = formatDistance(line.distanceMeters),
                                        style = MaterialTheme.typography.labelSmall,
                                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                                    )
                                }
                            }
                        } else if (line.minutesAway == 0) {
                            Text(
                                text = "< 1 分钟",
                                style = MaterialTheme.typography.labelLarge,
                                fontWeight = FontWeight.Bold,
                                color = MaterialTheme.colorScheme.primary,
                            )
                        } else if (line.stationsAway > 0) {
                            Text(
                                text = "${line.stationsAway}站",
                                style = MaterialTheme.typography.titleMedium,
                                fontWeight = FontWeight.Bold,
                                color = MaterialTheme.colorScheme.primary,
                            )
                            if (line.distanceMeters > 0) {
                                Text(
                                    text = formatDistance(line.distanceMeters),
                                    style = MaterialTheme.typography.labelSmall,
                                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                                )
                            }
                        }
                    }
                    ArrivalState.NoService -> {
                        Text(
                            text = "未运营",
                            style = MaterialTheme.typography.labelMedium,
                            color = MaterialTheme.colorScheme.outline,
                        )
                    }
                    ArrivalState.Unknown -> {}
                }
            }
        }
    }
}

private fun formatDistance(meters: Int): String {
    return if (meters >= 1000) {
        String.format("%.1fkm", meters / 1000.0)
    } else {
        "${meters}m"
    }
}

private fun formatStationName(name: String, alias: String?): String {
    return if (!alias.isNullOrBlank()) "$name($alias)" else name
}
