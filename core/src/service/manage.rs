use crate::domain::AppConfig;

use crate::domain::{CacheCategory, CacheStats, ManageError, ServiceInfo};

pub trait ManageService: Send + Sync {
    async fn load_config(&self) -> Result<AppConfig, ManageError>;
    async fn save_config(&self, config: AppConfig) -> Result<(), ManageError>;
    async fn switch_service(&self, service_id: &str) -> Result<ServiceInfo, ManageError>;
    async fn cache_stats(&self) -> Result<CacheStats, ManageError>;
    async fn clear_cache(&self) -> Result<(), ManageError>;
    async fn clear_cache_category(&self, category: CacheCategory) -> Result<(), ManageError>;
}
