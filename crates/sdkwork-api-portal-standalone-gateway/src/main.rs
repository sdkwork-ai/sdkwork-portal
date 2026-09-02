use sdkwork_api_portal_assembly::web_module;
use sdkwork_iam_web_adapter::{
    build_web_framework_builder, iam_web_request_context_resolver_from_env,
};
use sdkwork_web_bootstrap::{infra_public_path_prefixes, ApiModuleRegistry};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting SDKWork Portal API Server...");

    // The module owns the complete portal HTTP surface (API_ASSEMBLY_SPEC
    // §4.1.1); the host only installs it and publishes the composed router.
    let module = web_module().await.expect("portal API assembly failed");
    let manifest = module
        .contributions()
        .first()
        .map(|contribution| contribution.route_manifest.clone())
        .expect("portal web module exposes at least one surface contribution");
    let framework = build_web_framework_builder(
        iam_web_request_context_resolver_from_env().await,
        manifest,
        infra_public_path_prefixes(),
    );
    let mut module_registry = ApiModuleRegistry::new();
    module_registry.add_module(module);
    let hosted = module_registry
        .try_compose("SDKWork Portal API")
        .expect("portal API composition failed")
        .into_hosted(framework);
    let app = hosted
        .router
        .layer(sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_PORTAL_ENVIRONMENT"],
            &["SDKWORK_CORS_ALLOWED_ORIGINS"],
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
