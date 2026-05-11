use crate::domain::{LineDetailSnapshot, NetworkState};
use crate::models::{AppConfig, BusRoute};
use crux_core::capability::Operation;
use crux_core::macros::effect;
use serde::{Deserialize, Serialize};

use super::dto::{BootstrapData, NearbyData, ServiceSwitchData};
use super::view::CacheStatsView;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapOp;
impl Operation for BootstrapOp {
    type Output = BootstrapData;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveConfigOp {
    pub config: AppConfig,
}
impl Operation for SaveConfigOp {
    type Output = ();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLocationPermissionOp;
impl Operation for RequestLocationPermissionOp {
    type Output = bool;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchAutoLocationOp;
impl Operation for FetchAutoLocationOp {
    type Output = Result<(f64, f64), String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchNearbyOp {
    pub lat: f64,
    pub lng: f64,
    #[serde(default)]
    pub force: bool,
}
impl Operation for FetchNearbyOp {
    type Output = (Result<NearbyData, String>, NetworkState);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadAllRoutesOp;
impl Operation for LoadAllRoutesOp {
    type Output = Result<Vec<BusRoute>, String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchDetailOp {
    pub direction_id: String,
    #[serde(default)]
    pub target_order: u32,
}
impl Operation for FetchDetailOp {
    type Output = Result<LineDetailSnapshot, String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchServiceOp {
    pub service_id: String,
}
impl Operation for SwitchServiceOp {
    type Output = Result<ServiceSwitchData, String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadCacheStatsOp;
impl Operation for LoadCacheStatsOp {
    type Output = CacheStatsView;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearCacheOp;
impl Operation for ClearCacheOp {
    type Output = CacheStatsView;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearCacheCategoryOp {
    pub index: i32,
}
impl Operation for ClearCacheCategoryOp {
    type Output = CacheStatsView;
}

#[effect(typegen)]
pub enum Effect {
    Bootstrap(BootstrapOp),
    SaveConfig(SaveConfigOp),
    RequestLocationPermission(RequestLocationPermissionOp),
    FetchAutoLocation(FetchAutoLocationOp),
    FetchNearby(FetchNearbyOp),
    LoadAllRoutes(LoadAllRoutesOp),
    FetchDetail(FetchDetailOp),
    SwitchService(SwitchServiceOp),
    LoadCacheStats(LoadCacheStatsOp),
    ClearCache(ClearCacheOp),
    ClearCacheCategory(ClearCacheCategoryOp),
}
