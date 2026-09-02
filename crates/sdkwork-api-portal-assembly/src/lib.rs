//! API assembly for sdkwork-portal.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.
// SDKWORK-ASSEMBLY-LIB-CUSTOM: exports beyond the canonical materializer template.

mod bootstrap;
mod generated;

pub use bootstrap::{assemble_api_router, ApiAssembly, ApiAssemblyContext, assemble_api_router_with_pool, assemble_app_api_contribution, assemble_backend_api_contribution, web_module, web_module_with_context, web_module_with_pool};

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
