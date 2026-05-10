@file:OptIn(androidx.compose.material3.ExperimentalMaterial3Api::class)

package com.noctiro.wherebus.ui.pages

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material.icons.filled.LocationCity
import androidx.compose.material.icons.filled.Search
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.ListItem
import androidx.compose.material3.MaterialTheme
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

@Composable
fun CityPickerScreen(
    state: WhereBusUiState,
    onAction: (WhereBusAction) -> Unit,
    onNavigateBack: () -> Unit = {},
) {
    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("数据源选择") },
                navigationIcon = {
                    IconButton(onClick = onNavigateBack) {
                        Icon(Icons.AutoMirrored.Filled.ArrowBack, contentDescription = "返回")
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(containerColor = Color.Transparent),
            )
        },
    ) { padding ->
        LazyColumn(
            modifier = Modifier.fillMaxSize().padding(padding),
            contentPadding = PaddingValues(vertical = 8.dp),
        ) {
            item {
                OutlinedTextField(
                    value = state.cityPickerQuery,
                    onValueChange = { onAction(WhereBusAction.CityFilterChanged(it)) },
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp)
                        .padding(bottom = 8.dp),
                    label = { Text("搜索城市或数据源") },
                    singleLine = true,
                    leadingIcon = { Icon(Icons.Default.Search, contentDescription = null) },
                )
            }

            if (state.cityPickerGroups.isEmpty()) {
                item {
                    EmptyStateCard(
                        icon = Icons.Default.LocationCity,
                        title = "没有匹配结果",
                        message = "换一个关键词试试。",
                    )
                }
            } else {
                state.cityPickerGroups.forEach { group ->
                    stickyHeader(key = group.region) {
                        Text(
                            text = group.region,
                            style = MaterialTheme.typography.labelLarge,
                            color = MaterialTheme.colorScheme.primary,
                            modifier = Modifier
                                .fillMaxWidth()
                                .background(MaterialTheme.colorScheme.background.copy(alpha = 0.95f))
                                .padding(horizontal = 16.dp, vertical = 8.dp),
                        )
                    }
                    items(group.services, key = { it.id }) { service ->
                        ListItem(
                            headlineContent = { Text(service.cityName) },
                            supportingContent = if (service.providerName.isNotBlank()) {
                                { Text(service.providerName) }
                            } else null,
                            modifier = Modifier.clickable { onAction(WhereBusAction.SelectService(service.id)) },
                        )
                    }
                }
            }
        }
    }
}
