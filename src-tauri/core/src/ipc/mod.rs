use crate::traits::AppBuilder;
use tauri::Runtime;
use tauri_bindgen_host::ipc_router_wip::BuilderExt;
mod app;
mod builder;
mod traits;
use builder::RouterBuilder;

#[derive(Default)]
pub(self) struct GlobalContext {
    app_ctx: app::AppCtx,
}

pub struct Ipc;

impl<R: Runtime> AppBuilder<R> for Ipc {
    fn build(builder: tauri::Builder<R>) -> tauri::Builder<R> {
        let router_builder = RouterBuilder::new(GlobalContext::default()).nest::<app::AppCtx>();
        builder.ipc_router(router_builder.build().unwrap())
    }
}
