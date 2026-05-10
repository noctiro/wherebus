package com.noctiro.wherebus.ui.theme

import android.os.Build
import androidx.compose.material3.ColorScheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.dynamicDarkColorScheme
import androidx.compose.material3.dynamicLightColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalContext
import com.noctiro.wherebus.domain.ThemeMode

private val LightColors = lightColorScheme(
    primary = Seed,
    onPrimary = androidx.compose.ui.graphics.Color.White,
    primaryContainer = androidx.compose.ui.graphics.Color(0xFFCFFAF4),
    onPrimaryContainer = androidx.compose.ui.graphics.Color(0xFF00201D),
    secondary = androidx.compose.ui.graphics.Color(0xFF475569),
    onSecondary = androidx.compose.ui.graphics.Color.White,
    secondaryContainer = androidx.compose.ui.graphics.Color(0xFFE2E8F0),
    onSecondaryContainer = androidx.compose.ui.graphics.Color(0xFF111827),
    tertiary = Accent,
    onTertiary = androidx.compose.ui.graphics.Color.White,
    tertiaryContainer = androidx.compose.ui.graphics.Color(0xFFFDE7D5),
    onTertiaryContainer = androidx.compose.ui.graphics.Color(0xFF3A1A00),
    background = LightBackground,
    onBackground = androidx.compose.ui.graphics.Color(0xFF0F172A),
    surface = LightSurface,
    onSurface = androidx.compose.ui.graphics.Color(0xFF0F172A),
    surfaceVariant = LightSurfaceVariant,
    onSurfaceVariant = androidx.compose.ui.graphics.Color(0xFF475569),
    outline = androidx.compose.ui.graphics.Color(0xFF94A3B8),
)

private val DarkColors = darkColorScheme(
    primary = SeedDark,
    onPrimary = androidx.compose.ui.graphics.Color(0xFF00201D),
    primaryContainer = androidx.compose.ui.graphics.Color(0xFF164E45),
    onPrimaryContainer = androidx.compose.ui.graphics.Color(0xFFCFFAF4),
    secondary = androidx.compose.ui.graphics.Color(0xFFCBD5E1),
    onSecondary = androidx.compose.ui.graphics.Color(0xFF111827),
    secondaryContainer = androidx.compose.ui.graphics.Color(0xFF334155),
    onSecondaryContainer = androidx.compose.ui.graphics.Color(0xFFE2E8F0),
    tertiary = Accent,
    onTertiary = androidx.compose.ui.graphics.Color(0xFF1F1205),
    tertiaryContainer = androidx.compose.ui.graphics.Color(0xFF5A3412),
    onTertiaryContainer = androidx.compose.ui.graphics.Color(0xFFFDE7D5),
    background = DarkBackground,
    onBackground = androidx.compose.ui.graphics.Color(0xFFE2E8F0),
    surface = DarkSurface,
    onSurface = androidx.compose.ui.graphics.Color(0xFFE2E8F0),
    surfaceVariant = DarkSurfaceVariant,
    onSurfaceVariant = androidx.compose.ui.graphics.Color(0xFFCBD5E1),
    outline = androidx.compose.ui.graphics.Color(0xFF64748B),
)

@Composable
fun WhereBusTheme(
    themeMode: ThemeMode = ThemeMode.System,
    content: @Composable () -> Unit,
) {
    val darkTheme = when (themeMode) {
        ThemeMode.System -> androidx.compose.foundation.isSystemInDarkTheme()
        ThemeMode.Light -> false
        ThemeMode.Dark -> true
    }

    val colorScheme: ColorScheme = when {
        Build.VERSION.SDK_INT >= Build.VERSION_CODES.S -> {
            val context = LocalContext.current
            if (darkTheme) dynamicDarkColorScheme(context) else dynamicLightColorScheme(context)
        }
        darkTheme -> DarkColors
        else -> LightColors
    }

    MaterialTheme(
        colorScheme = colorScheme,
        typography = Typography,
        content = content,
    )
}
