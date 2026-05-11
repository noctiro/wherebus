use crate::models::{
    AppConfig, ArrivalEstimate, BusPosition, BusRoute, CongestionLevel, ContactInfo, CrowdLevel,
    Fare, LineSummary, LocationMode, RouteSegment, RunState, StopStatus,
};
use crate::support::pinyin_match::matches_pinyin;
use crux_core::{App, Command};

use super::dto::BootstrapData;
use super::effect::{
    BootstrapOp, ClearCacheCategoryOp, ClearCacheOp, Effect, FetchAutoLocationOp, FetchDetailOp,
    FetchNearbyOp, LoadAllRoutesOp, LoadCacheStatsOp, RequestLocationPermissionOp, SaveConfigOp,
    SwitchServiceOp,
};
use super::event::Event;
use super::model::{Model, SelectedLine};
use super::view::{
    AppView, BusItemView, CacheCategoryView, CacheStatsView, CityItemView, LineCardView,
    StationItemView,
};
use crate::domain::LineDetailSnapshot;

const CACHE_CATEGORIES: &[(&str, &str)] = &[
    ("附近站点", "站点位置与距离信息"),
    ("站点线路", "各站点经过的线路"),
    ("线路详情", "线路站点、首末班时间等"),
    ("全部线路", "城市线路列表"),
];

pub struct WherebusApp;

impl Default for WherebusApp {
    fn default() -> Self {
        let _ = crate::app::event::touch_event_variants();
        Self
    }
}

impl App for WherebusApp {
    type Event = Event;
    type Model = Model;
    type ViewModel = AppView;
    type Effect = Effect;

    fn update(&self, event: Event, model: &mut Model) -> Command<Effect, Event> {
        match event {
            Event::Boot => command_bootstrap(),
            Event::BootLoaded(data) => apply_bootstrap(model, data),
            Event::WelcomeStart => {
                model.config.onboarding_done = true;
                model.show_welcome = false;
                save_config_command(model.config.clone())
            }
            Event::ChangePage(page) => {
                model.current_page = page;
                Command::done()
            }
            Event::WelcomeRequestLocation => request_permission_command(),
            Event::LocationPermissionLoaded(granted) => {
                model.location_permission = granted;
                model.welcome_location_granted = granted;
                if granted && model.config.location_mode == LocationMode::Auto {
                    fetch_auto_location_command()
                } else {
                    Command::done()
                }
            }
            Event::NearbyRefresh => start_nearby_force_refresh(model),
            Event::NearbyLoaded(result, net_state) => {
                model.nearby_loading = false;
                model.nearby_net_state = net_state;
                match result {
                    Ok(data) => {
                        model.nearby_error.clear();
                        model.nearby_items = data.items;
                        model.nearby_lines = data.nearby_lines;
                        refresh_search_results(model);
                    }
                    Err(err) => {
                        model.nearby_error = err;
                    }
                }
                Command::done()
            }
            Event::NearbyLineSelected(flat_idx) => open_line_from_nearby(model, flat_idx),
            Event::SearchChanged(query) => {
                model.search_query = query;
                refresh_search_results(model);
                Command::done()
            }
            Event::SearchLineSelected(idx) => open_line_from_search(model, idx),
            Event::AllRoutesLoaded(result) => {
                model.search_loading = false;
                if let Ok(routes) = result {
                    model.all_routes = routes;
                    refresh_search_results(model);
                }
                Command::done()
            }
            Event::DetailGoBack => {
                model.show_line_detail = false;
                Command::done()
            }
            Event::DetailSwitchDirection => {
                let Some(current) = model.current_line.clone() else {
                    return Command::done();
                };
                let Some(reverse) = current.reverse_id else {
                    return Command::done();
                };
                begin_detail_load(model, current.name, reverse, 0)
            }
            Event::DetailRefresh => {
                let Some(current) = model.current_line.clone() else {
                    return Command::done();
                };
                begin_detail_load(
                    model,
                    current.name,
                    current.direction_id,
                    current.target_order,
                )
            }
            Event::DetailLoaded(result) => {
                model.detail_loading = false;
                match result {
                    Ok(data) => {
                        if let Some(rt) = data.realtime.as_ref() {
                            sync_nearby_run_state(
                                model,
                                &model.detail_line_name.clone(),
                                Some(rt.run_state),
                            );
                        }
                        populate_detail(model, data);
                    }
                    Err(err) => model.detail_error = err,
                }
                Command::done()
            }
            Event::SettingsSaveLocation(lat_str, lng_str) => {
                let lat: f64 = match lat_str.trim().parse() {
                    Ok(value) => value,
                    Err(_) => {
                        model.settings_status = "纬度格式错误".into();
                        return Command::done();
                    }
                };
                let lng: f64 = match lng_str.trim().parse() {
                    Ok(value) => value,
                    Err(_) => {
                        model.settings_status = "经度格式错误".into();
                        return Command::done();
                    }
                };

                model.config.manual_location = Some((lat, lng));
                model.config.location_mode = LocationMode::Manual;
                model.settings_status = format!("已保存: {lat}, {lng}");

                Command::all([
                    save_config_command(model.config.clone()),
                    start_nearby_refresh(model),
                ])
            }
            Event::SettingsLocationModeChanged(is_auto) => {
                model.config.location_mode = if is_auto {
                    LocationMode::Auto
                } else {
                    LocationMode::Manual
                };

                let mut commands = vec![save_config_command(model.config.clone())];
                if is_auto {
                    commands.push(fetch_auto_location_command());
                } else if model.config.manual_location.is_some() {
                    commands.push(start_nearby_refresh(model));
                }

                Command::all(commands)
            }
            Event::AutoLocationLoaded(result) => match result {
                Ok((lat, lng)) => {
                    model.config.auto_location = Some((lat, lng));
                    model.settings_auto_available = true;
                    model.settings_auto_status = format!("已定位: {lat:.4}, {lng:.4}");
                    if model.config.location_mode == LocationMode::Auto {
                        start_nearby_refresh(model)
                    } else {
                        Command::done()
                    }
                }
                Err(err) => {
                    model.settings_auto_available = false;
                    model.settings_auto_status = format!("获取失败: {err}");
                    Command::done()
                }
            },
            Event::SettingsOpenCityPicker => {
                model.show_city_picker = true;
                Command::done()
            }
            Event::CityPickerGoBack => {
                model.show_city_picker = false;
                model.city_picker_query.clear();
                model.city_picker_cities = model.city_picker_all.clone();
                Command::done()
            }
            Event::CityPickerSelected(service_id) => {
                model.config.service_id = service_id.clone();
                model.show_city_picker = false;
                switch_service_command(service_id)
            }
            Event::CityPickerFilter(query) => {
                model.city_picker_query = query.clone();
                let needle = query.to_lowercase();
                model.city_picker_cities = if needle.is_empty() {
                    model.city_picker_all.clone()
                } else {
                    let mut result = Vec::new();
                    let mut pending_header: Option<&CityItemView> = None;
                    let mut header_matches = false;
                    for item in &model.city_picker_all {
                        if item.is_header {
                            pending_header = Some(item);
                            header_matches = item.name.to_lowercase().contains(&needle)
                                || matches_pinyin(&item.name, &needle);
                        } else if header_matches
                            || item.name.to_lowercase().contains(&needle)
                            || item.provider.to_lowercase().contains(&needle)
                            || matches_pinyin(&item.name, &needle)
                            || matches_pinyin(&item.provider, &needle)
                        {
                            if let Some(h) = pending_header.take() {
                                result.push(h.clone());
                            }
                            result.push(item.clone());
                        }
                    }
                    result
                };
                Command::done()
            }
            Event::ServiceSwitched(result) => match result {
                Ok(data) => {
                    model.provider_name = data.provider_name;
                    model.city_label = data.city_label;
                    model.nearby_items.clear();
                    model.nearby_lines.clear();
                    model.all_routes.clear();
                    model.search_results.clear();
                    model.search_lookup.clear();
                    model.show_line_detail = false;

                    let mut commands = vec![load_all_routes_command()];
                    if model.config.effective_location().is_some() {
                        commands.push(start_nearby_refresh(model));
                    }
                    Command::all(commands)
                }
                Err(err) => {
                    model.settings_status = err;
                    Command::done()
                }
            },
            Event::SettingsThemeChanged(idx) => {
                model.config.theme_index = idx;
                save_config_command(model.config.clone())
            }
            Event::SettingsClearCache => clear_cache_command(),
            Event::SettingsOpenCacheManager => {
                model.show_cache_manager = true;
                load_cache_stats_command()
            }
            Event::CacheManagerGoBack => {
                model.show_cache_manager = false;
                Command::done()
            }
            Event::CacheManagerClearCategory(idx) => clear_cache_category_command(idx),
            Event::CacheManagerClearAll => clear_cache_command(),
            Event::CacheStatsLoaded(stats) => {
                model.cache_stats = stats;
                Command::done()
            }
        }
    }

    fn view(&self, model: &Model) -> AppView {
        let (cache_manager_categories, cache_manager_total) = cache_sections(&model.cache_stats);
        AppView {
            show_welcome: model.show_welcome,
            welcome_show_permissions: model.welcome_show_permissions,
            welcome_location_granted: model.welcome_location_granted,
            current_page: model.current_page,
            show_line_detail: model.show_line_detail,
            show_city_picker: model.show_city_picker,
            show_cache_manager: model.show_cache_manager,
            nearby_loading: model.nearby_loading,
            nearby_error: model.nearby_error.clone(),
            nearby_items: model.nearby_items.clone(),
            nearby_net_state: model.nearby_net_state,
            search_query: model.search_query.clone(),
            search_results: model.search_results.clone(),
            search_loading: model.search_loading,
            detail_line_name: model.detail_line_name.clone(),
            detail_direction: model
                .current_line
                .as_ref()
                .map(|line| line.direction_id.clone())
                .unwrap_or_default(),
            detail_direction_label: model
                .detail_snapshot
                .as_ref()
                .map(|s| {
                    format!(
                        "{} → {}",
                        s.detail
                            .topology
                            .stations
                            .first()
                            .map(|st| st.name.as_str())
                            .unwrap_or(""),
                        s.detail
                            .topology
                            .stations
                            .last()
                            .map(|st| st.name.as_str())
                            .unwrap_or("")
                    )
                })
                .unwrap_or_default(),
            detail_can_switch: model
                .current_line
                .as_ref()
                .and_then(|l| l.reverse_id.as_ref())
                .is_some(),
            detail_comments: model
                .detail_snapshot
                .as_ref()
                .and_then(|s| s.detail.meta.notes.clone())
                .unwrap_or_default(),
            detail_loading: model.detail_loading,
            detail_error: model.detail_error.clone(),
            detail_stations: model.detail_stations.clone(),
            detail_first_time: model
                .detail_snapshot
                .as_ref()
                .and_then(|s| s.detail.meta.first_service.map(|t| t.to_string()))
                .unwrap_or_default(),
            detail_last_time: model
                .detail_snapshot
                .as_ref()
                .and_then(|s| s.detail.meta.last_service.map(|t| t.to_string()))
                .unwrap_or_default(),
            detail_price: model
                .detail_snapshot
                .as_ref()
                .map(|s| match &s.detail.meta.fare {
                    Fare::Text(v) => v.clone(),
                    Fare::Unknown => String::new(),
                })
                .unwrap_or_default(),
            detail_company: model
                .detail_snapshot
                .as_ref()
                .and_then(|s| s.detail.meta.company.clone())
                .unwrap_or_default(),
            detail_phone: model
                .detail_snapshot
                .as_ref()
                .map(|s| match &s.detail.meta.contact {
                    ContactInfo::Phone(v) => v.clone(),
                    ContactInfo::Unknown => String::new(),
                })
                .unwrap_or_default(),
            detail_plan_time: model
                .detail_snapshot
                .as_ref()
                .and_then(|s| s.realtime.as_ref())
                .and_then(|rt| rt.plan_time.clone())
                .unwrap_or_default(),
            detail_run_state: model
                .detail_snapshot
                .as_ref()
                .and_then(|s| s.realtime.as_ref())
                .map(|rt| rt.run_state),
            settings_current_city: model.city_label.clone(),
            settings_provider_name: model.provider_name.clone(),
            settings_location_auto: model.config.location_mode == LocationMode::Auto,
            settings_auto_available: model.settings_auto_available,
            settings_auto_status: model.settings_auto_status.clone(),
            settings_lat: model
                .config
                .manual_location
                .map(|(lat, _)| lat.to_string())
                .unwrap_or_default(),
            settings_lng: model
                .config
                .manual_location
                .map(|(_, lng)| lng.to_string())
                .unwrap_or_default(),
            settings_status: model.settings_status.clone(),
            settings_theme_index: model.config.theme_index,
            settings_cache_info: cache_info(&model.cache_stats),
            settings_app_version: env!("CARGO_PKG_VERSION").into(),
            settings_build_target: option_env!("BUILD_TARGET").unwrap_or("").into(),
            settings_build_profile: option_env!("BUILD_PROFILE").unwrap_or("").into(),
            city_picker_query: model.city_picker_query.clone(),
            city_picker_cities: model.city_picker_cities.clone(),
            cache_manager_categories,
            cache_manager_total,
        }
    }
}

fn command_bootstrap() -> Command<Effect, Event> {
    Command::request_from_shell(BootstrapOp).then_send(Event::BootLoaded)
}

fn request_permission_command() -> Command<Effect, Event> {
    Command::request_from_shell(RequestLocationPermissionOp)
        .then_send(Event::LocationPermissionLoaded)
}

fn fetch_auto_location_command() -> Command<Effect, Event> {
    Command::request_from_shell(FetchAutoLocationOp).then_send(Event::AutoLocationLoaded)
}

fn load_all_routes_command() -> Command<Effect, Event> {
    Command::request_from_shell(LoadAllRoutesOp).then_send(Event::AllRoutesLoaded)
}

fn save_config_command(config: AppConfig) -> Command<Effect, Event> {
    Command::notify_shell(SaveConfigOp { config }).into()
}

fn switch_service_command(service_id: String) -> Command<Effect, Event> {
    Command::request_from_shell(SwitchServiceOp { service_id }).then_send(Event::ServiceSwitched)
}

fn load_cache_stats_command() -> Command<Effect, Event> {
    Command::request_from_shell(LoadCacheStatsOp).then_send(Event::CacheStatsLoaded)
}

fn clear_cache_command() -> Command<Effect, Event> {
    Command::request_from_shell(ClearCacheOp).then_send(Event::CacheStatsLoaded)
}

fn clear_cache_category_command(index: i32) -> Command<Effect, Event> {
    Command::request_from_shell(ClearCacheCategoryOp { index }).then_send(Event::CacheStatsLoaded)
}

fn start_nearby_refresh(model: &mut Model) -> Command<Effect, Event> {
    start_nearby_fetch(model, false)
}

fn start_nearby_force_refresh(model: &mut Model) -> Command<Effect, Event> {
    start_nearby_fetch(model, true)
}

fn start_nearby_fetch(model: &mut Model, force: bool) -> Command<Effect, Event> {
    let Some((lat, lng)) = model.config.effective_location() else {
        model.nearby_loading = false;
        model.nearby_error = "请先设置位置".into();
        return Command::done();
    };

    let (lat, lng) = crate::support::coord::wgs84_to_gcj02(lat, lng);
    model.nearby_loading = true;
    model.nearby_error.clear();

    Command::request_from_shell(FetchNearbyOp { lat, lng, force })
        .then_send(|(result, net_state)| Event::NearbyLoaded(result, net_state))
}

fn begin_detail_load(
    model: &mut Model,
    name: String,
    direction_id: String,
    target_order: u32,
) -> Command<Effect, Event> {
    model.current_line = Some(SelectedLine {
        name: name.clone(),
        direction_id: direction_id.clone(),
        reverse_id: None,
        target_order,
    });
    model.show_line_detail = true;
    model.detail_line_name = name;
    model.detail_loading = true;
    model.detail_error.clear();
    model.detail_snapshot = None;
    model.detail_stations.clear();

    Command::request_from_shell(FetchDetailOp {
        direction_id,
        target_order,
    })
    .then_send(Event::DetailLoaded)
}

fn open_line_from_nearby(model: &mut Model, flat_idx: i32) -> Command<Effect, Event> {
    let Some(Some(line)) = model.nearby_lines.get(flat_idx as usize).cloned() else {
        return Command::done();
    };
    begin_detail_load(model, line.name, line.direction_id, line.station_order)
}

fn open_line_from_search(model: &mut Model, idx: i32) -> Command<Effect, Event> {
    let Some(line) = model.search_lookup.get(idx as usize).cloned() else {
        return Command::done();
    };
    begin_detail_load(model, line.name, line.direction_id, line.target_order)
}

fn apply_bootstrap(model: &mut Model, data: BootstrapData) -> Command<Effect, Event> {
    let onboarding_done = data.config.onboarding_done;
    let location_mode = data.config.location_mode.clone();
    let manual_location = data.config.manual_location;

    model.config = data.config;
    model.provider_name = data.provider_name;
    model.city_label = data.city_label;
    model.location_permission = data.location_permission;
    model.show_welcome = !onboarding_done;
    model.welcome_show_permissions = false;
    model.welcome_location_granted = data.location_permission;
    model.city_picker_all = data.city_picker_cities.clone();
    model.city_picker_cities = data.city_picker_cities;
    model.cache_stats = data.cache_stats;

    let mut commands = vec![load_all_routes_command()];
    match location_mode {
        LocationMode::Auto if data.location_permission => {
            model.settings_auto_status = "正在获取位置...".into();
            commands.push(fetch_auto_location_command());
        }
        LocationMode::Manual if manual_location.is_some() => {
            commands.push(start_nearby_refresh(model));
        }
        _ => {}
    }

    Command::all(commands)
}

fn refresh_search_results(model: &mut Model) {
    let query = model.search_query.to_lowercase();

    if query.is_empty() {
        let cards: Vec<LineCardView> = model
            .nearby_lines
            .iter()
            .filter_map(|line| line.as_ref())
            .map(line_card_from_summary)
            .collect();

        model.search_lookup = model
            .nearby_lines
            .iter()
            .filter_map(|line| line.as_ref().map(selected_line_from_summary))
            .collect();
        apply_search_cards(model, cards);
        return;
    }

    let filtered: Vec<BusRoute> = model
        .all_routes
        .iter()
        .filter(|route| {
            route.name.to_lowercase().contains(&query) || matches_pinyin(&route.name, &query)
        })
        .take(50)
        .cloned()
        .collect();

    model.search_lookup = filtered.iter().map(selected_line_from_route).collect();

    apply_search_cards(
        model,
        filtered
            .iter()
            .map(line_card_from_route)
            .collect::<Vec<_>>(),
    );
}

fn apply_search_cards(model: &mut Model, cards: Vec<LineCardView>) {
    model.search_results = cards;
}

fn line_card_from_summary(line: &LineSummary) -> LineCardView {
    let arrival_text = match &line.arrival {
        ArrivalEstimate::Arriving => "即将到站".to_string(),
        ArrivalEstimate::Approaching {
            stations_away,
            minutes_away,
            ..
        } => match minutes_away {
            Some(m) => format!("{}站 约{}分钟", stations_away, m),
            None => format!("{}站", stations_away),
        },
        ArrivalEstimate::NoService => "未运营".to_string(),
        ArrivalEstimate::Unknown => String::new(),
    };
    LineCardView {
        line_name: line.name.clone(),
        direction_id: line.direction_id.clone(),
        endpoints: line.endpoints.clone(),
        arrival_text,
        arrival_distance: String::new(),
        station_name: String::new(),
        station_order: line.station_order as i32,
        company: String::new(),
    }
}

fn line_card_from_route(route: &BusRoute) -> LineCardView {
    LineCardView {
        line_name: route.name.clone(),
        direction_id: route.direction_id.clone(),
        endpoints: route.endpoints.clone(),
        arrival_text: String::new(),
        arrival_distance: String::new(),
        station_name: String::new(),
        station_order: 0,
        company: route.company.clone().unwrap_or_default(),
    }
}

fn selected_line_from_summary(line: &LineSummary) -> SelectedLine {
    SelectedLine {
        name: line.name.clone(),
        direction_id: line.direction_id.clone(),
        reverse_id: None,
        target_order: line.station_order,
    }
}

fn selected_line_from_route(route: &BusRoute) -> SelectedLine {
    SelectedLine {
        name: route.name.clone(),
        direction_id: route.direction_id.clone(),
        reverse_id: None,
        target_order: 0,
    }
}

fn sync_nearby_run_state(model: &mut Model, line_name: &str, run_state: Option<RunState>) {
    for item in model.nearby_items.iter_mut() {
        if !item.is_header && item.line.line_name == line_name {
            item.line.run_state = run_state;
        }
    }
}

fn populate_detail(model: &mut Model, data: LineDetailSnapshot) {
    model.detail_error.clear();

    if let Some(ref mut current) = model.current_line {
        current.reverse_id = data.detail.reverse_id.clone();
        current.direction_id = data.detail.direction_id.clone();
    }

    let detail = &data.detail;
    let realtime = data.realtime.as_ref();

    let (buses, segments): (&[BusPosition], &[RouteSegment]) = if let Some(rt) = realtime {
        (&rt.buses, &rt.segments)
    } else {
        (&[], &[])
    };

    let mut buses_by_station: std::collections::HashMap<u32, Vec<BusItemView>> =
        std::collections::HashMap::new();
    for bus in buses {
        buses_by_station
            .entry(bus.station_index)
            .or_default()
            .push(BusItemView {
                bus_id: bus.bus_id.clone(),
                is_arriving: bus.is_arriving,
                distance_m: bus.distance_to_station.map(|d| d as i32).unwrap_or(-1),
                travel_time_secs: bus.travel_time_secs.map(|t| t as i32).unwrap_or(-1),
                crowd_level: match bus.crowd_status {
                    CrowdLevel::Unknown => None,
                    other => Some(other),
                },
                state_desc: bus.state_description.clone().unwrap_or_default(),
            });
    }

    let congestion_map: Vec<Option<CongestionLevel>> = segments
        .iter()
        .map(|segment| match segment.congestion {
            CongestionLevel::Unknown => None,
            other => Some(other),
        })
        .collect();
    let distance_map: Vec<i32> = segments
        .iter()
        .map(|segment| segment.distance.map(|d| d as i32).unwrap_or(0))
        .collect();
    let speed_map: Vec<i32> = segments
        .iter()
        .map(|segment| segment.speed.map(|speed| speed as i32).unwrap_or(0))
        .collect();
    let time_map: Vec<i32> = segments
        .iter()
        .map(|segment| match (segment.distance, segment.speed) {
            (Some(distance), Some(speed)) if speed > 0.0 => (distance * 3.6 / speed) as i32,
            _ => 0,
        })
        .collect();

    let user_location = model
        .config
        .effective_location()
        .map(|(lat, lng)| crate::support::coord::wgs84_to_gcj02(lat, lng));
    let user_distances: Vec<i32> = detail
        .topology
        .stations
        .iter()
        .map(|station| match user_location {
            Some((lat, lng)) if station.lat != 0.0 && station.lng != 0.0 => {
                crate::support::coord::haversine_distance_m(lat, lng, station.lat, station.lng)
                    as i32
            }
            _ => 0,
        })
        .collect();
    let nearest_idx = user_distances
        .iter()
        .enumerate()
        .filter(|(_, distance)| **distance > 0)
        .min_by_key(|(_, distance)| **distance)
        .map(|(index, _)| index);

    model.detail_stations = detail
        .topology
        .stations
        .iter()
        .enumerate()
        .map(|(index, station)| {
            let station_buses = buses_by_station
                .get(&station.order)
                .cloned()
                .unwrap_or_default();
            let arrival = realtime.and_then(|rt| {
                rt.station_arrivals
                    .iter()
                    .find(|arrival| arrival.station_index == station.order)
            });
            let has_bus = !station_buses.is_empty()
                || arrival
                    .is_some_and(|arrival| arrival.arriving_count > 0 || arrival.leaving_count > 0);
            let bus_info = arrival
                .map(|arrival| {
                    let mut parts = Vec::new();
                    if arrival.arriving_count > 0 {
                        parts.push(format!("{}辆到站", arrival.arriving_count));
                    }
                    if arrival.leaving_count > 0 {
                        parts.push(format!("{}辆离站", arrival.leaving_count));
                    }
                    parts.join(" ")
                })
                .unwrap_or_default();

            StationItemView {
                name: station.name.clone(),
                alias: station.alias.clone().unwrap_or_default(),
                order: station.order as i32,
                is_current: false,
                is_nearest: nearest_idx == Some(index),
                distance_to_user_m: user_distances[index],
                has_bus,
                bus_info,
                stop_type: match station.status {
                    StopStatus::Normal => None,
                    other => Some(other),
                },
                congestion: congestion_map.get(index).copied().unwrap_or(None),
                prev_congestion: if index > 0 {
                    congestion_map.get(index - 1).copied().unwrap_or(None)
                } else {
                    None
                },
                segment_distance_m: distance_map.get(index).copied().unwrap_or(0),
                segment_speed_kmh: speed_map.get(index).copied().unwrap_or(0),
                segment_time_secs: time_map.get(index).copied().unwrap_or(0),
                buses: station_buses,
            }
        })
        .collect();

    model.detail_snapshot = Some(data);
}

fn cache_sections(stats: &CacheStatsView) -> (Vec<CacheCategoryView>, String) {
    let counts = [
        stats.stations,
        stats.station_lines,
        stats.line_detail,
        stats.all_lines,
    ];
    let total: usize = counts.iter().sum();
    let categories = CACHE_CATEGORIES
        .iter()
        .enumerate()
        .map(|(index, (label, description))| CacheCategoryView {
            label: (*label).into(),
            count: counts[index] as i32,
            description: (*description).into(),
        })
        .collect();
    let total_info = if total > 0 {
        format!("共 {total} 条缓存")
    } else {
        "无缓存数据".into()
    };
    (categories, total_info)
}

fn cache_info(stats: &CacheStatsView) -> String {
    let total = stats.stations + stats.station_lines + stats.line_detail + stats.all_lines;
    if total > 0 {
        format!("{total} 条缓存")
    } else {
        String::new()
    }
}
