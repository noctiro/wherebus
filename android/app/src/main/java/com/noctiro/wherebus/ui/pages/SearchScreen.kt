@file:OptIn(androidx.compose.material3.ExperimentalMaterial3Api::class)

package com.noctiro.wherebus.ui.pages

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.Route
import androidx.compose.material.icons.filled.Search
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.TopAppBarDefaults
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import com.noctiro.wherebus.ui.WhereBusAction
import com.noctiro.wherebus.ui.WhereBusUiState
import com.noctiro.wherebus.ui.components.EmptyStateCard
import com.noctiro.wherebus.ui.components.LineCard
import com.noctiro.wherebus.ui.components.TextButtonAction

@Composable
fun SearchScreen(
    state: WhereBusUiState,
    onAction: (WhereBusAction) -> Unit,
    onNavigateBack: () -> Unit = {},
) {
    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("搜索线路") },
                navigationIcon = {
                    IconButton(onClick = onNavigateBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "返回")
                    }
                },
                actions = {
                    if (state.searchQuery.isNotBlank()) {
                        TextButtonAction(text = "清空") {
                            onAction(WhereBusAction.SearchChanged(""))
                        }
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(containerColor = Color.Transparent),
            )
        },
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding),
        ) {
            OutlinedTextField(
                value = state.searchQuery,
                onValueChange = { onAction(WhereBusAction.SearchChanged(it)) },
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 16.dp)
                    .padding(bottom = 12.dp),
                placeholder = { Text("线路名、方向、站点") },
                singleLine = true,
                leadingIcon = { Icon(Icons.Default.Search, contentDescription = null) },
            )

            if (state.searchLoading) {
                LinearProgressIndicator(modifier = Modifier.fillMaxWidth())
            }

            if (state.searchQuery.isBlank()) {
                EmptyStateCard(
                    icon = Icons.Default.Search,
                    title = "输入关键词开始搜索",
                    message = "支持线路编号、站点名或方向。",
                )
            } else if (state.searchResults.isEmpty() && !state.searchLoading) {
                EmptyStateCard(
                    icon = Icons.Default.Route,
                    title = "没有匹配结果",
                    message = "试试其他关键词。",
                )
            } else {
                LazyVerticalGrid(
                    columns = GridCells.Adaptive(320.dp),
                    modifier = Modifier.fillMaxSize(),
                    contentPadding = PaddingValues(horizontal = 16.dp, vertical = 8.dp),
                    horizontalArrangement = Arrangement.spacedBy(12.dp),
                    verticalArrangement = Arrangement.spacedBy(12.dp),
                ) {
                    items(state.searchResults.size) { index ->
                        LineCard(
                            card = state.searchResults[index],
                            onClick = { onAction(WhereBusAction.OpenSearchLine(index)) },
                        )
                    }
                }
            }
        }
    }
}
