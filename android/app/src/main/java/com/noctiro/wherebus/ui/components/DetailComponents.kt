package com.noctiro.wherebus.ui.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.DirectionsBus
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
