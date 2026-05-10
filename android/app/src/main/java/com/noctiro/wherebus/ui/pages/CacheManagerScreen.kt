@file:OptIn(androidx.compose.material3.ExperimentalMaterial3Api::class)

package com.noctiro.wherebus.ui.pages

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.Cached
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.ListItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import com.noctiro.wherebus.ui.WhereBusAction
import com.noctiro.wherebus.ui.WhereBusUiState
import com.noctiro.wherebus.ui.components.TextButtonAction

@Composable
fun CacheManagerScreen(
    state: WhereBusUiState,
    onAction: (WhereBusAction) -> Unit,
    onNavigateBack: () -> Unit = {},
) {
    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("缓存管理") },
                navigationIcon = {
                    IconButton(onClick = onNavigateBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "返回")
                    }
                },
                actions = {
                    TextButtonAction(text = "清空全部", onClick = { onAction(WhereBusAction.ClearCache) })
                },
                colors = TopAppBarDefaults.topAppBarColors(containerColor = Color.Transparent),
            )
        },
    ) { padding ->
        LazyColumn(
            modifier = Modifier.fillMaxSize().padding(padding),
        ) {
            item {
                ListItem(
                    headlineContent = { Text("缓存总量") },
                    supportingContent = { Text("${state.cacheStats.total()} 条 · 清理后会自动重建") },
                    leadingContent = { Icon(Icons.Default.Cached, contentDescription = null) },
                )
            }
            items(state.cacheCategories) { category ->
                ListItem(
                    headlineContent = { Text(category.label) },
                    supportingContent = { Text("${category.count} 条 · ${category.description}") },
                    trailingContent = {
                        TextButtonAction(text = "清除") {
                            onAction(WhereBusAction.ClearCacheCategory(category.category))
                        }
                    },
                )
            }
        }
    }
}
