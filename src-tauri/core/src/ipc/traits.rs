use tauri_bindgen_host::{ipc_router_wip::Router, Result};

pub trait RouterCtx<G>
where
    Self: Sized + Send + Sync,
{
    fn register_router(router: &mut Router<G>) -> Result<()>;
}
