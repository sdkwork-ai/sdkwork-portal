use axum::routing::get;
use axum::Router;
use sdkwork_portal_service_host::PortalServiceHost;
use std::sync::Arc;

use crate::handlers;
#[derive(Clone)]
pub struct PortalBackendState {
    pub host: Arc<PortalServiceHost>,
}

pub fn build_portal_backend_router(host: Arc<PortalServiceHost>) -> Router {
    let state = PortalBackendState { host };
    Router::new()
        .route(
            "/backend/v3/api/portal/preferences",
            get(handlers::list_preferences_admin),
        )
        .with_state(state)
}
