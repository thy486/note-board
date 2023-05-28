use tauri_bindgen_host::ipc_router_wip::Router;

use crate::wits::app::{add_to_router, App};

use super::{traits::RouterCtx, GlobalContext};

#[derive(Default)]
pub struct AppCtx {
    last_greet: Option<String>,
}

impl App for AppCtx {
    fn greet(&mut self, name: String) -> String {
        self.last_greet = Some(name.clone());
        format!("Hello, {}! You've been greeted from Rust!", name)
    }

    fn last_greet(&mut self) -> Option<String> {
        self.last_greet.to_owned()
    }
}

impl RouterCtx<GlobalContext> for AppCtx {
    #[inline(always)]
    fn register_router(router: &mut Router<GlobalContext>) -> tauri_bindgen_host::Result<()> {
        add_to_router(router, |ctx| &mut ctx.app_ctx)
    }
}
