// DetailScreenHeader.kt — Fixed header components for DetailScreen
@file:OptIn(ExperimentalMaterial3Api::class)

package com.noctiro.wherebus.ui.pages

import androidx.compose.animation.animateColorAsState
import androidx.compose.animation.core.FastOutSlowInEasing
import androidx.compose.animation.core.Spring
import androidx.compose.animation.core.animateDpAsState
import androidx.compose.animation.core.animateFloat
import androidx.compose.animation.core.infiniteRepeatable
import androidx.compose.animation.core.rememberInfiniteTransition
import androidx.compose.animation.core.spring
import androidx.compose.animation.core.tween
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.DirectionsBus
import androidx.compose.material.icons.filled.Info
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.SwapVert
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableLongStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.drawWithContent
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.noctiro.wherebus.ui.components.BusCardData
import kotlinx.coroutines.delay

@Composable
internal fun DetailFixedHeader(
    lineName: String,
    directionLabel: String,
    isLoading: Boolean,
    hasData: Boolean,
    canSwitch: Boolean,
    lastUpdateTime: Long,
    busCards: List<BusCardData>,
    selectedBusId: String?,
    onBack: () -> Unit,
    onRefresh: () -> Unit,
    onSwitch: () -> Unit,
    onInfoClick: () -> Unit,
    onBusCardClick: (BusCardData) -> Unit,
) {
    val surfaceColor = MaterialTheme.colorScheme.surface

    Column(modifier = Modifier.background(surfaceColor)) {
        TopAppBar(
            title = {
                Column {
                    Text(
                        text = lineName,
                        style = MaterialTheme.typography.titleLarge,
                        fontWeight = FontWeight.Medium,
                    )
                    if (directionLabel.isNotBlank()) {
                        Text(
                            text = directionLabel,
                            style = MaterialTheme.typography.labelMedium,
                            color = MaterialTheme.colorScheme.onSurfaceVariant,
                            maxLines = 1,
                            overflow = TextOverflow.Ellipsis,
                        )
                    }
                }
            },
            navigationIcon = {
                IconButton(onClick = onBack) {
                    Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "返回")
                }
            },
            actions = {
                IconButton(onClick = onInfoClick) {
                    Icon(Icons.Default.Info, contentDescription = "线路信息")
                }
                if (canSwitch) {
                    IconButton(onClick = onSwitch) {
                        Icon(Icons.Default.SwapVert, contentDescription = "切换方向")
                    }
                }
                IconButton(onClick = onRefresh) {
                    Icon(Icons.Default.Refresh, contentDescription = "刷新")
                }
            },
            colors = TopAppBarDefaults.topAppBarColors(
                containerColor = surfaceColor,
            ),
        )

        TimestampRow(
            lastUpdateTime = lastUpdateTime,
            isRefreshing = isLoading && hasData,
        )

        if (busCards.isNotEmpty()) {
            BusCardsRow(
                cards = busCards,
                selectedBusId = selectedBusId,
                onCardClick = onBusCardClick,
            )
        } else if (hasData) {
            Box(
                modifier = Modifier
                    .fillMaxWidth()
                    .height(48.dp),
                contentAlignment = Alignment.Center,
            ) {
                Text(
                    text = "暂无在途车辆",
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }

        HorizontalDivider(
            thickness = 0.5.dp,
            color = MaterialTheme.colorScheme.outlineVariant,
        )
    }
}

@Composable
private fun TimestampRow(
    lastUpdateTime: Long,
    isRefreshing: Boolean,
) {
    var now by remember { mutableLongStateOf(System.currentTimeMillis()) }
    LaunchedEffect(Unit) {
        while (true) {
            now = System.currentTimeMillis()
            delay(1000)
        }
    }

    val elapsedSec = ((now - lastUpdateTime) / 1000).coerceAtLeast(0)

    val dotColor = when {
        elapsedSec <= 60 -> Color(0xFF4CAF50)
        elapsedSec <= 120 -> Color(0xFFFFA726)
        else -> Color(0xFFEF5350)
    }
    val textColor = when {
        elapsedSec <= 60 -> MaterialTheme.colorScheme.onSurfaceVariant
        elapsedSec <= 120 -> Color(0xFFFFA726)
        else -> Color(0xFFEF5350)
    }

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .height(24.dp)
            .padding(horizontal = 16.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Box(
            modifier = Modifier
                .size(6.dp)
                .clip(CircleShape)
                .background(dotColor),
        )
        Spacer(modifier = Modifier.width(6.dp))
        Text(
            text = buildString {
                append("${elapsedSec}秒前更新")
                if (elapsedSec > 120) append(" · 数据可能过时")
            },
            style = MaterialTheme.typography.labelSmall,
            color = textColor,
        )
        Spacer(modifier = Modifier.weight(1f))
        if (isRefreshing) {
            CircularProgressIndicator(
                modifier = Modifier.size(12.dp),
                strokeWidth = 1.5.dp,
                color = MaterialTheme.colorScheme.primary,
            )
        }
    }
}

@Composable
private fun BusCardsRow(
    cards: List<BusCardData>,
    selectedBusId: String?,
    onCardClick: (BusCardData) -> Unit,
) {
    val scrollState = rememberScrollState()
    val surfaceColor = MaterialTheme.colorScheme.surface

    Box(
        modifier = Modifier
            .fillMaxWidth(),
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .horizontalScroll(scrollState)
                .padding(horizontal = 14.dp, vertical = 8.dp),
            horizontalArrangement = Arrangement.spacedBy(10.dp),
        ) {
            cards.forEach { card ->
                BusCard(
                    card = card,
                    isSelected = card.busId == selectedBusId,
                    onClick = { onCardClick(card) },
                )
            }
        }

        val showFade = scrollState.maxValue > 0 &&
            scrollState.value < scrollState.maxValue
        if (showFade) {
            Box(
                modifier = Modifier
                    .align(Alignment.CenterEnd)
                    .width(32.dp)
                    .matchParentSize()
                    .background(
                        Brush.horizontalGradient(
                            colors = listOf(Color.Transparent, surfaceColor),
                        )
                    ),
            )
        }
    }
}

@Composable
private fun BusCard(
    card: BusCardData,
    isSelected: Boolean,
    onClick: () -> Unit,
) {
    val isAtStation = card.isAtStation
    val isApproaching = !isAtStation && card.etaMinutes in 1..15
    val isFar = !isAtStation && card.etaMinutes > 15

    val infiniteTransition = rememberInfiniteTransition(label = "pulse")

    val pulseShadow by if (isAtStation) {
        infiniteTransition.animateFloat(
            initialValue = 0f,
            targetValue = 6f,
            animationSpec = infiniteRepeatable(tween(2000)),
            label = "shadowPulse",
        )
    } else {
        remember { androidx.compose.runtime.mutableFloatStateOf(0f) }
    }

    val pulseAlpha by if (isAtStation) {
        infiniteTransition.animateFloat(
            initialValue = 1f,
            targetValue = 0.3f,
            animationSpec = infiniteRepeatable(tween(2000)),
            label = "alphaPulse",
        )
    } else {
        remember { androidx.compose.runtime.mutableFloatStateOf(1f) }
    }

    val bgColor = when {
        isAtStation -> MaterialTheme.colorScheme.errorContainer
        isApproaching && isSelected -> MaterialTheme.colorScheme.primary
        isApproaching -> MaterialTheme.colorScheme.primaryContainer
        else -> MaterialTheme.colorScheme.surfaceContainerLow
    }

    val contentColor = when {
        isAtStation -> MaterialTheme.colorScheme.error
        isApproaching && isSelected -> MaterialTheme.colorScheme.onPrimary
        isApproaching -> MaterialTheme.colorScheme.onPrimaryContainer
        else -> MaterialTheme.colorScheme.onSurface
    }

    val subtleColor = when {
        isAtStation -> MaterialTheme.colorScheme.error.copy(alpha = 0.7f)
        isApproaching && isSelected -> MaterialTheme.colorScheme.onPrimary.copy(alpha = 0.7f)
        isApproaching -> MaterialTheme.colorScheme.onPrimaryContainer.copy(alpha = 0.7f)
        else -> MaterialTheme.colorScheme.onSurfaceVariant
    }

    val dotColor = when {
        isAtStation -> MaterialTheme.colorScheme.error
        isApproaching -> MaterialTheme.colorScheme.primary
        else -> MaterialTheme.colorScheme.outlineVariant
    }

    val borderWidth by animateDpAsState(
        targetValue = if (isFar && isSelected) 1.5.dp else 0.5.dp,
        label = "border",
    )
    val borderColor by animateColorAsState(
        targetValue = if (isFar && isSelected)
            MaterialTheme.colorScheme.primary
        else
            MaterialTheme.colorScheme.outlineVariant,
        label = "borderColor",
    )

    Surface(
        modifier = Modifier
            .width(148.dp)
            .then(
                if (isAtStation) Modifier.shadow(pulseShadow.dp, RoundedCornerShape(14.dp))
                else Modifier
            )
            .clickable(enabled = !isAtStation, onClick = onClick),
        shape = RoundedCornerShape(14.dp),
        color = bgColor,
        border = if (isFar) androidx.compose.foundation.BorderStroke(borderWidth, borderColor)
        else null,
    ) {
        Column(modifier = Modifier.padding(horizontal = 11.dp, vertical = 13.dp)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    text = card.busId.ifBlank { "—" },
                    style = MaterialTheme.typography.labelSmall,
                    color = subtleColor,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis,
                )
                Box(
                    modifier = Modifier
                        .size(6.dp)
                        .clip(CircleShape)
                        .background(dotColor.copy(alpha = if (isAtStation) pulseAlpha else 1f)),
                )
            }

            Spacer(modifier = Modifier.height(8.dp))

            if (isAtStation) {
                Text(
                    text = "在站",
                    style = MaterialTheme.typography.headlineSmall,
                    fontWeight = FontWeight.SemiBold,
                    color = contentColor,
                )
            } else {
                Row(verticalAlignment = Alignment.Bottom) {
                    Text(
                        text = card.etaMinutes.toString(),
                        style = if (isApproaching) MaterialTheme.typography.headlineSmall
                        else MaterialTheme.typography.headlineMedium,
                        fontWeight = if (isApproaching) FontWeight.SemiBold else FontWeight.Medium,
                        color = contentColor,
                    )
                    Spacer(modifier = Modifier.width(2.dp))
                    Text(
                        text = "分钟",
                        style = MaterialTheme.typography.labelMedium,
                        color = subtleColor,
                        modifier = Modifier.padding(bottom = 2.dp),
                    )
                }
            }

            Spacer(modifier = Modifier.height(2.dp))

            Text(
                text = if (isAtStation) "立刻上车"
                else "现在：${card.currentStationName}",
                style = MaterialTheme.typography.labelSmall,
                color = subtleColor,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
            )
        }
    }
}
