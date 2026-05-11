// DetailScreenTimeline.kt — Station timeline components for DetailScreen
package com.noctiro.wherebus.ui.pages

import androidx.compose.animation.animateColorAsState
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.animateDpAsState
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.animation.core.spring
import androidx.compose.animation.core.tween
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.IntrinsicSize
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.DirectionsBus
import androidx.compose.material.icons.filled.LocationOn
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.noctiro.wherebus.domain.CongestionLevel
import com.noctiro.wherebus.domain.StationItemUi
import com.noctiro.wherebus.ui.components.StopBadge

internal fun congestionColor(level: CongestionLevel?, fallback: Color): Color = when (level) {
    CongestionLevel.Smooth -> Color(0xFF4CAF50)
    CongestionLevel.Slow -> Color(0xFFFFA726)
    CongestionLevel.Congested -> Color(0xFFEF5350)
    null -> fallback
}

@Composable
internal fun StationTimelineRow(
    station: StationItemUi,
    displayOrder: Int,
    isFirst: Boolean,
    isLast: Boolean,
    isNearest: Boolean,
    isSelected: Boolean,
    isPassed: Boolean,
    nearestBusEta: Int?,
    selectedBusEta: Int?,
    onSelect: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val primaryColor = MaterialTheme.colorScheme.primary
    val outlineVariantColor = MaterialTheme.colorScheme.outlineVariant
    val tertiaryColor = MaterialTheme.colorScheme.tertiary

    val trackColor = congestionColor(
        station.prevCongestion,
        if (isPassed || isNearest) primaryColor else outlineVariantColor,
    )
    val bottomTrackColor = congestionColor(
        station.congestion,
        if (isPassed) primaryColor else outlineVariantColor,
    )

    val cardAlpha by animateFloatAsState(
        targetValue = if (isPassed && !isNearest) 0.5f else 1f,
        label = "cardAlpha",
    )

    val hasBus = station.buses.isNotEmpty()
    val dotColor = when {
        hasBus -> tertiaryColor
        isNearest -> primaryColor
        isPassed -> primaryColor
        else -> outlineVariantColor
    }
    val dotSize by animateDpAsState(
        targetValue = when {
            isNearest -> 20.dp
            hasBus -> 14.dp
            else -> 10.dp
        },
        animationSpec = spring(stiffness = Spring.StiffnessMediumLow),
        label = "dotSize",
    )

    val bgColor by animateColorAsState(
        targetValue = when {
            isNearest -> MaterialTheme.colorScheme.primaryContainer
            isSelected -> MaterialTheme.colorScheme.secondaryContainer
            else -> Color.Transparent
        },
        animationSpec = tween(200),
        label = "bgColor",
    )

    Row(
        modifier = modifier
            .fillMaxWidth()
            .height(IntrinsicSize.Min),
    ) {
        // Left: timeline track
        Box(
            modifier = Modifier
                .width(28.dp)
                .fillMaxHeight(),
        ) {
            if (!isFirst) {
                Box(
                    modifier = Modifier
                        .width(2.dp)
                        .height(22.dp)
                        .align(Alignment.TopCenter)
                        .background(trackColor),
                )
            }
            if (!isLast) {
                Box(
                    modifier = Modifier
                        .width(2.dp)
                        .fillMaxHeight()
                        .padding(top = 22.dp)
                        .align(Alignment.TopCenter)
                        .background(bottomTrackColor),
                )
            }
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
                    contentAlignment = Alignment.Center,
                ) {
                    if (isNearest) {
                        Icon(
                            imageVector = Icons.Default.LocationOn,
                            contentDescription = null,
                            modifier = Modifier.size(11.dp),
                            tint = MaterialTheme.colorScheme.onPrimary,
                        )
                    }
                }
            }
        }

        Spacer(modifier = Modifier.width(6.dp))

        // Middle + Right content
        Surface(
            modifier = Modifier
                .weight(1f)
                .alpha(cardAlpha)
                .padding(vertical = 3.dp)
                .clickable(onClick = onSelect),
            shape = RoundedCornerShape(12.dp),
            color = bgColor,
        ) {
            Row(
                modifier = Modifier.padding(10.dp, 12.dp),
                verticalAlignment = Alignment.Top,
            ) {
                Text(
                    text = displayOrder.toString().padStart(2, '0'),
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant.copy(alpha = 0.5f),
                )
                Spacer(modifier = Modifier.width(8.dp))
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

                if (hasBus) {
                    Spacer(modifier = Modifier.width(6.dp))
                } else when {
                    isNearest && nearestBusEta != null -> {
                        Spacer(modifier = Modifier.width(6.dp))
                        EtaBadge(
                            text = "${nearestBusEta}分",
                            containerColor = MaterialTheme.colorScheme.primary,
                            contentColor = MaterialTheme.colorScheme.onPrimary,
                        )
                    }
                    isSelected && selectedBusEta != null -> {
                        Spacer(modifier = Modifier.width(6.dp))
                        EtaBadge(
                            text = "${selectedBusEta}分",
                            containerColor = MaterialTheme.colorScheme.secondaryContainer,
                            contentColor = MaterialTheme.colorScheme.onSecondaryContainer,
                        )
                    }
                    isFirst -> {
                        Spacer(modifier = Modifier.width(6.dp))
                        StopBadge("起点")
                    }
                    isLast -> {
                        Spacer(modifier = Modifier.width(6.dp))
                        StopBadge("终点")
                    }
                }
            }
        }
    }
}

@Composable
private fun EtaBadge(
    text: String,
    containerColor: Color,
    contentColor: Color,
    showBusIcon: Boolean = false,
) {
    Surface(
        shape = RoundedCornerShape(50),
        color = containerColor,
    ) {
        Row(
            modifier = Modifier.padding(horizontal = 8.dp, vertical = 3.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(3.dp),
        ) {
            if (showBusIcon) {
                Icon(
                    imageVector = Icons.Default.DirectionsBus,
                    contentDescription = null,
                    modifier = Modifier.size(12.dp),
                    tint = contentColor,
                )
            }
            Text(
                text = text,
                style = MaterialTheme.typography.labelSmall,
                fontWeight = FontWeight.Medium,
                color = contentColor,
            )
        }
    }
}

@Composable
private fun BusIdChip(
    busId: String,
    tertiaryColor: Color,
) {
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
                tint = MaterialTheme.colorScheme.onTertiary,
            )
            Text(
                text = busId,
                fontSize = 10.sp,
                fontWeight = FontWeight.Medium,
                color = MaterialTheme.colorScheme.onTertiary,
            )
        }
    }
}

@Composable
internal fun BusOnTrackRow(
    buses: List<com.noctiro.wherebus.domain.BusItemUi>,
    isArriving: Boolean,
    trackColor: Color,
    modifier: Modifier = Modifier,
) {
    val tertiaryColor = MaterialTheme.colorScheme.tertiary

    Row(
        modifier = modifier
            .fillMaxWidth()
            .height(IntrinsicSize.Min),
    ) {
        Box(
            modifier = Modifier
                .width(28.dp)
                .fillMaxHeight(),
        ) {
            Box(
                modifier = Modifier
                    .width(2.dp)
                    .fillMaxHeight()
                    .align(Alignment.TopCenter)
                    .background(trackColor),
            )
            Icon(
                imageVector = Icons.Default.DirectionsBus,
                contentDescription = null,
                modifier = Modifier
                    .size(18.dp)
                    .align(Alignment.Center)
                    .clip(CircleShape)
                    .background(tertiaryColor)
                    .padding(2.dp),
                tint = MaterialTheme.colorScheme.onTertiary,
            )
        }

        Spacer(modifier = Modifier.width(6.dp))

        @OptIn(ExperimentalLayoutApi::class)
        FlowRow(
            modifier = Modifier
                .weight(1f)
                .padding(vertical = 4.dp, horizontal = 8.dp),
            horizontalArrangement = Arrangement.spacedBy(4.dp, Alignment.End),
            verticalArrangement = Arrangement.spacedBy(3.dp),
        ) {
            buses.forEach { bus ->
                BusIdChip(busId = bus.busId, tertiaryColor = tertiaryColor)
            }
        }
    }
}
