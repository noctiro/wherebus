use std::sync::Arc;

use async_trait::async_trait;

use crate::models::City;
use crate::provider::{ProviderError, BusDataProvider};
use crate::domain::transit::{Station, LineSummary, LineDetail, RealTimeData, BusRoute};

#[cfg(debug_assertions)]
use super::debug;
use super::{chelaile, mygolbs};

#[derive(Clone, Debug)]
pub struct ServiceEntry {
    pub id: &'static str,
    pub city: City,
    pub provider: &'static str,
}

pub fn available_services() -> Vec<ServiceEntry> {
    let mut services: Vec<ServiceEntry> = mygolbs::cities::CITIES
        .iter()
        .map(|city| ServiceEntry {
            id: city.api_city_key,
            city: city.city,
            provider: "掌上公交",
        })
        .collect();

    services.extend(chelaile::cities::CITIES.iter().map(|city| ServiceEntry {
        id: city.city_id,
        city: city.city,
        provider: "车来了",
    }));

    #[cfg(debug_assertions)]
    services.push(ServiceEntry {
        id: debug::SERVICE_ID,
        city: City::Beijing,
        provider: "Debug",
    });

    services
}

pub fn create_provider(service_id: &str) -> Arc<dyn BusDataProvider> {
    if service_id.is_empty() {
        return Arc::new(NullProvider);
    }

    #[cfg(debug_assertions)]
    if service_id == debug::SERVICE_ID {
        return Arc::new(debug::DebugProvider);
    }

    if let Some(city) = chelaile::cities::CITIES
        .iter()
        .find(|city| city.city_id == service_id)
    {
        let inner = chelaile::ChelaileProvider::new(city.city_id, city.city.name()).unwrap();
        return Arc::new(inner);
    }

    if let Some(city) = mygolbs::cities::CITIES
        .iter()
        .find(|city| city.api_city_key == service_id)
    {
        let inner = mygolbs::MygolbsProvider::from_city(city).unwrap();
        return Arc::new(inner);
    }

    Arc::new(NullProvider)
}

struct NullProvider;

#[async_trait]
impl BusDataProvider for NullProvider {
    fn provider_name(&self) -> &str { "" }
    fn city_name(&self) -> &str { "" }

    async fn nearby_stations(&self, _lat: f64, _lng: f64) -> Result<Vec<Station>, ProviderError> {
        Ok(vec![])
    }
    async fn station_lines(&self, _station: &str, _lat: f64, _lng: f64) -> Result<Vec<LineSummary>, ProviderError> {
        Ok(vec![])
    }
    async fn line_detail(&self, _key: &str) -> Result<LineDetail, ProviderError> {
        Err(ProviderError::Server("未选择数据源".into()))
    }
    async fn realtime(&self, _key: &str, _target_order: u32) -> Result<RealTimeData, ProviderError> {
        Err(ProviderError::Server("未选择数据源".into()))
    }
    async fn all_lines(&self) -> Result<Vec<BusRoute>, ProviderError> {
        Ok(vec![])
    }
}
