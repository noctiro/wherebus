pub mod manage;
pub mod query;

use std::sync::Arc;

use parking_lot::Mutex;

use crate::domain::AppConfig;
use crate::kernel::RuntimeFacade;

pub use crate::domain::{
    CacheCategory, CacheStats, LineDetailSnapshot, ManageError, NearbySnapshot, NetworkState,
    QueryError, ServiceInfo,
};
pub use manage::ManageService;
pub use query::QueryService;

pub struct Services {
    inner: Arc<Mutex<Arc<RuntimeFacade>>>,
}

impl Services {
    pub fn open() -> anyhow::Result<Self> {
        Ok(Self {
            inner: Arc::new(Mutex::new(Arc::new(RuntimeFacade::open()?))),
        })
    }
}

impl Services {
    fn with_inner<R>(&self, f: impl FnOnce(&RuntimeFacade) -> R) -> R {
        let guard = self.inner.lock();
        f(&guard)
    }

    fn clone_inner(&self) -> Arc<RuntimeFacade> {
        self.inner.lock().clone()
    }
}

#[allow(refining_impl_trait)]
impl QueryService for Services {
    async fn available_services(&self) -> Vec<ServiceInfo> {
        self.with_inner(|inner| inner.available_services())
    }

    async fn current_service(&self) -> ServiceInfo {
        self.with_inner(|inner| inner.current_service()).into()
    }

    async fn network_state(&self) -> NetworkState {
        self.with_inner(|inner| inner.network_state())
    }

    async fn nearby(&self, lat: f64, lng: f64) -> Result<NearbySnapshot, QueryError> {
        let inner = self.clone_inner();
        inner
            .nearby_snapshot(lat, lng)
            .await
            .map_err(QueryError::from)
    }

    async fn refresh_nearby(&self, lat: f64, lng: f64) -> Result<NearbySnapshot, QueryError> {
        let inner = self.clone_inner();
        inner
            .refresh_nearby(lat, lng)
            .await
            .map_err(QueryError::from)
    }

    async fn all_routes(&self) -> Result<Vec<crate::models::BusRoute>, QueryError> {
        let inner = self.clone_inner();
        inner
            .all_lines()
            .await
            .map_err(|err| QueryError::from(err.to_string()))
    }

    async fn line_detail(
        &self,
        direction_id: &str,
        target_order: u32,
    ) -> Result<LineDetailSnapshot, QueryError> {
        let inner = self.clone_inner();
        inner
            .line_detail_snapshot(direction_id, target_order)
            .await
            .map_err(QueryError::from)
    }
}

#[allow(refining_impl_trait)]
impl ManageService for Services {
    async fn load_config(&self) -> Result<AppConfig, ManageError> {
        let inner = self.clone_inner();
        inner
            .load_config()
            .map_err(|e| ManageError::Message(e.to_string()))
    }

    async fn save_config(&self, config: AppConfig) -> Result<(), ManageError> {
        let inner = self.clone_inner();
        inner
            .save_config(&config)
            .map_err(|e| ManageError::Message(e.to_string()))
    }

    async fn switch_service(&self, service_id: &str) -> Result<ServiceInfo, ManageError> {
        let next = self.with_inner(|inner| inner.switch_provider(service_id));
        let mut config = next
            .load_config()
            .map_err(|e| ManageError::Message(e.to_string()))?;
        config.service_id = service_id.to_string();
        let _ = next.save_config(&config);
        let info = ServiceInfo::from(next.current_service());
        *self.inner.lock() = next;
        Ok(info)
    }

    async fn cache_stats(&self) -> Result<CacheStats, ManageError> {
        let inner = self.clone_inner();
        Ok(inner.cache_stats())
    }

    async fn clear_cache(&self) -> Result<(), ManageError> {
        let inner = self.clone_inner();
        inner
            .clear_cache()
            .map_err(|e| ManageError::Message(e.to_string()))
    }

    async fn clear_cache_category(&self, category: CacheCategory) -> Result<(), ManageError> {
        let inner = self.clone_inner();
        let category = match category {
            CacheCategory::Stations => crate::domain::CacheCategory::Stations,
            CacheCategory::StationLines => crate::domain::CacheCategory::StationLines,
            CacheCategory::LineDetail => crate::domain::CacheCategory::LineDetail,
            CacheCategory::AllLines => crate::domain::CacheCategory::AllLines,
        };
        inner
            .clear_cache_category(category)
            .map_err(|e| ManageError::Message(e.to_string()))
    }
}
