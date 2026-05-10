use crate::models::{AppConfig, BusRoute, LineDetail, RealTimeData, RunState};
use serde::{Deserialize, Serialize};

use crate::domain::NetworkState;

use crate::domain::LineDetailSnapshot;

use super::dto::{BootstrapData, NearbyData, ServiceSwitchData};
use super::view::CacheStatsView;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Boot,
    BootLoaded(BootstrapData),
    WelcomeStart,
    ChangePage(i32),
    WelcomeRequestLocation,
    LocationPermissionLoaded(bool),
    NearbyRefresh,
    NearbyLoaded(Result<NearbyData, String>, NetworkState),
    NearbyLineSelected(i32),
    SearchChanged(String),
    SearchLineSelected(i32),
    AllRoutesLoaded(Result<Vec<BusRoute>, String>),
    DetailGoBack,
    DetailSwitchDirection,
    DetailRefresh,
    DetailLoaded(Result<LineDetailSnapshot, String>),
    SettingsSaveLocation(String, String),
    SettingsLocationModeChanged(bool),
    AutoLocationLoaded(Result<(f64, f64), String>),
    SettingsOpenCityPicker,
    CityPickerGoBack,
    CityPickerSelected(String),
    CityPickerFilter(String),
    ServiceSwitched(Result<ServiceSwitchData, String>),
    SettingsThemeChanged(i32),
    SettingsClearCache,
    SettingsOpenCacheManager,
    CacheManagerGoBack,
    CacheManagerClearCategory(i32),
    CacheManagerClearAll,
    CacheStatsLoaded(CacheStatsView),
}

pub(crate) fn touch_event_variants() -> usize {
    let events = [
        Event::Boot,
        Event::BootLoaded(BootstrapData {
            config: AppConfig::default(),
            provider_name: String::new(),
            city_label: String::new(),
            location_permission: false,
            city_picker_cities: Vec::new(),
            cache_stats: CacheStatsView::default(),
        }),
        Event::WelcomeStart,
        Event::ChangePage(0),
        Event::WelcomeRequestLocation,
        Event::LocationPermissionLoaded(false),
        Event::NearbyRefresh,
        Event::NearbyLoaded(
            Ok(NearbyData {
                items: Vec::new(),
                nearby_lines: Vec::new(),
            }),
            NetworkState::Online,
        ),
        Event::NearbyLineSelected(0),
        Event::SearchChanged(String::new()),
        Event::SearchLineSelected(0),
        Event::AllRoutesLoaded(Ok(Vec::new())),
        Event::DetailGoBack,
        Event::DetailSwitchDirection,
        Event::DetailRefresh,
        Event::DetailLoaded(Ok(LineDetailSnapshot {
            detail: LineDetail {
                id: None,
                name: String::new(),
                direction_id: String::new(),
                reverse_id: None,
                topology: crate::models::RouteTopology {
                    start: crate::models::Terminal::named(""),
                    end: crate::models::Terminal::named(""),
                    stations: Vec::new(),
                    track_points: Vec::new(),
                },
                meta: crate::models::LineMeta::default(),
            },
            realtime: Some(RealTimeData {
                run_state: RunState::NoRealtime,
                plan_time: None,
                buses: Vec::new(),
                station_arrivals: Vec::new(),
                segments: Vec::new(),
                arrival_estimates: Vec::new(),
            }),
        })),
        Event::SettingsSaveLocation(String::new(), String::new()),
        Event::SettingsLocationModeChanged(false),
        Event::AutoLocationLoaded(Ok((0.0, 0.0))),
        Event::SettingsOpenCityPicker,
        Event::CityPickerGoBack,
        Event::CityPickerSelected(String::new()),
        Event::CityPickerFilter(String::new()),
        Event::ServiceSwitched(Ok(ServiceSwitchData {
            provider_name: String::new(),
            city_label: String::new(),
        })),
        Event::SettingsThemeChanged(0),
        Event::SettingsClearCache,
        Event::SettingsOpenCacheManager,
        Event::CacheManagerGoBack,
        Event::CacheManagerClearCategory(0),
        Event::CacheManagerClearAll,
        Event::CacheStatsLoaded(CacheStatsView::default()),
    ];
    events.len()
}
