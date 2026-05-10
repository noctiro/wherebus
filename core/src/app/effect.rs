use crate::models::{AppConfig, BusRoute};
use crate::domain::LineDetailSnapshot;
use crux_core::{
    Request,
    bridge::{FfiFormat, ResolveSerialized},
    capability::Operation,
};
use serde::{Deserialize, Serialize};

use crate::domain::NetworkState;

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

#[derive(Debug)]
pub enum Effect {
    Bootstrap(Request<BootstrapOp>),
    SaveConfig(Request<SaveConfigOp>),
    RequestLocationPermission(Request<RequestLocationPermissionOp>),
    FetchAutoLocation(Request<FetchAutoLocationOp>),
    FetchNearby(Request<FetchNearbyOp>),
    LoadAllRoutes(Request<LoadAllRoutesOp>),
    FetchDetail(Request<FetchDetailOp>),
    SwitchService(Request<SwitchServiceOp>),
    LoadCacheStats(Request<LoadCacheStatsOp>),
    ClearCache(Request<ClearCacheOp>),
    ClearCacheCategory(Request<ClearCacheCategoryOp>),
}

impl crux_core::Effect for Effect {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectFfi {
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

impl crux_core::EffectFFI for Effect {
    type Ffi = EffectFfi;

    fn serialize<T: FfiFormat>(self) -> (Self::Ffi, ResolveSerialized<T>) {
        match self {
            Self::Bootstrap(request) => request.serialize(EffectFfi::Bootstrap),
            Self::SaveConfig(request) => request.serialize(EffectFfi::SaveConfig),
            Self::RequestLocationPermission(request) => {
                request.serialize(EffectFfi::RequestLocationPermission)
            }
            Self::FetchAutoLocation(request) => request.serialize(EffectFfi::FetchAutoLocation),
            Self::FetchNearby(request) => request.serialize(EffectFfi::FetchNearby),
            Self::LoadAllRoutes(request) => request.serialize(EffectFfi::LoadAllRoutes),
            Self::FetchDetail(request) => request.serialize(EffectFfi::FetchDetail),
            Self::SwitchService(request) => request.serialize(EffectFfi::SwitchService),
            Self::LoadCacheStats(request) => request.serialize(EffectFfi::LoadCacheStats),
            Self::ClearCache(request) => request.serialize(EffectFfi::ClearCache),
            Self::ClearCacheCategory(request) => request.serialize(EffectFfi::ClearCacheCategory),
        }
    }
}

impl From<Request<BootstrapOp>> for Effect {
    fn from(value: Request<BootstrapOp>) -> Self {
        Self::Bootstrap(value)
    }
}

impl From<Request<SaveConfigOp>> for Effect {
    fn from(value: Request<SaveConfigOp>) -> Self {
        Self::SaveConfig(value)
    }
}

impl From<Request<RequestLocationPermissionOp>> for Effect {
    fn from(value: Request<RequestLocationPermissionOp>) -> Self {
        Self::RequestLocationPermission(value)
    }
}

impl From<Request<FetchAutoLocationOp>> for Effect {
    fn from(value: Request<FetchAutoLocationOp>) -> Self {
        Self::FetchAutoLocation(value)
    }
}

impl From<Request<FetchNearbyOp>> for Effect {
    fn from(value: Request<FetchNearbyOp>) -> Self {
        Self::FetchNearby(value)
    }
}

impl From<Request<LoadAllRoutesOp>> for Effect {
    fn from(value: Request<LoadAllRoutesOp>) -> Self {
        Self::LoadAllRoutes(value)
    }
}

impl From<Request<FetchDetailOp>> for Effect {
    fn from(value: Request<FetchDetailOp>) -> Self {
        Self::FetchDetail(value)
    }
}

impl From<Request<SwitchServiceOp>> for Effect {
    fn from(value: Request<SwitchServiceOp>) -> Self {
        Self::SwitchService(value)
    }
}

impl From<Request<LoadCacheStatsOp>> for Effect {
    fn from(value: Request<LoadCacheStatsOp>) -> Self {
        Self::LoadCacheStats(value)
    }
}

impl From<Request<ClearCacheOp>> for Effect {
    fn from(value: Request<ClearCacheOp>) -> Self {
        Self::ClearCache(value)
    }
}

impl From<Request<ClearCacheCategoryOp>> for Effect {
    fn from(value: Request<ClearCacheCategoryOp>) -> Self {
        Self::ClearCacheCategory(value)
    }
}
