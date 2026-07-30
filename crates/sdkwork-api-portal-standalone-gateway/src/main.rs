use sdkwork_api_portal_assembly::assemble_api_router;
use sdkwork_iam_web_adapter::{
    build_web_framework_builder, iam_web_request_context_resolver_from_env,
};
use sdkwork_portal_service_host::PortalServiceHost;
use sdkwork_web_bootstrap::{infra_public_path_prefixes, ComposedApiAssembly};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting SDKWork Portal API Server...");

    let host = Arc::new(PortalServiceHost::new().await);
    let assembly = assemble_api_router(host)
        .await
        .expect("portal API assembly failed");
    let framework = build_web_framework_builder(
        iam_web_request_context_resolver_from_env().await,
        assembly.route_manifest.clone(),
        infra_public_path_prefixes(),
    );
    let hosted = ComposedApiAssembly::try_compose("SDKWork Portal API", vec![assembly])
        .expect("portal API composition failed")
        .into_hosted(framework);
    let app = hosted
        .router
        .layer(sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_PORTAL_ENVIRONMENT"],
            &[
                "SDKWORK_PORTAL_CORS_ALLOWED_ORIGINS",
                "SDKWORK_CORS_ALLOWED_ORIGINS",
            ],
        ));

    let addr = std::env::var("PORTAL_API_BIND").unwrap_or_else(|_| "0.0.0.0:18091".to_owned());
    tracing::info!("Portal API server listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind portal server");
    axum::serve(listener, app)
        .await
        .expect("serve portal server");
}
