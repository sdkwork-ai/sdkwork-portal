//! Backend API route boundary for SDKWork Portal.

use std::sync::Arc;

use axum::Router;
use sdkwork_portal_service_host::PortalServiceHost;
use sdkwork_web_core::HttpRouteManifest;

pub mod handlers;
pub mod http_route_manifest;
pub mod routes;

pub use http_route_manifest::backend_route_manifest;
pub use routes::build_portal_backend_router;

pub fn gateway_route_manifest() -> HttpRouteManifest {
    backend_route_manifest()
}

pub async fn gateway_mount(host: Arc<PortalServiceHost>) -> Router {
    build_portal_backend_router(host)
}
