//! API assembly for sdkwork-portal.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.

mod bootstrap;
mod generated;

pub use bootstrap::{assemble_api_router, assemble_api_router_with_pool, ApiAssembly, ApiAssemblyContext, assemble_app_api_contribution, assemble_backend_api_contribution};

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
