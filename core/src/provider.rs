pub(crate) mod chelaile;
#[cfg(debug_assertions)]
pub(crate) mod debug;
pub(crate) mod mygolbs;
pub(crate) mod registry;

use async_trait::async_trait;
use std::fmt;

use crate::models::*;

pub(crate) use registry::{available_services, create_provider};

#[derive(Debug, Clone)]
pub(crate) enum ProviderError {
    Network(String),
    Parse(String),
    Server(String),
}

impl fmt::Display for ProviderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Network(e) => write!(f, "网络错误: {e}"),
            Self::Parse(e) => write!(f, "解析错误: {e}"),
            Self::Server(e) => write!(f, "服务端错误: {e}"),
        }
    }
}

impl std::error::Error for ProviderError {}

#[async_trait]
pub(crate) trait BusDataProvider: Send + Sync {
    fn provider_name(&self) -> &str;
    fn city_name(&self) -> &str;

    async fn nearby_stations(&self, lat: f64, lng: f64) -> Result<Vec<Station>, ProviderError>;
    async fn station_lines(
        &self,
        station: &str,
        lat: f64,
        lng: f64,
    ) -> Result<Vec<LineSummary>, ProviderError>;
    async fn line_detail(&self, key: &str) -> Result<LineDetail, ProviderError>;
    async fn realtime(
        &self,
        key: &str,
        target_order: u32,
    ) -> Result<RealTimeData, ProviderError>;
    async fn all_lines(&self) -> Result<Vec<BusRoute>, ProviderError>;
}
