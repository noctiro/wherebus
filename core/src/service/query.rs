use crate::domain::{LineDetailSnapshot, NearbySnapshot, NetworkState, QueryError, ServiceInfo};

pub trait QueryService: Send + Sync {
    async fn available_services(&self) -> Vec<ServiceInfo>;
    async fn current_service(&self) -> ServiceInfo;
    async fn network_state(&self) -> NetworkState;
    async fn nearby(&self, lat: f64, lng: f64) -> Result<NearbySnapshot, QueryError>;
    async fn refresh_nearby(&self, lat: f64, lng: f64) -> Result<NearbySnapshot, QueryError>;
    async fn all_routes(&self) -> Result<Vec<crate::models::BusRoute>, QueryError>;
    async fn line_detail(
        &self,
        direction_id: &str,
        target_order: u32,
    ) -> Result<LineDetailSnapshot, QueryError>;
}
