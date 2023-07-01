use std::sync::Mutex;
use tauri_bindgen_host::ipc_router_wip::Router;

use crate::wits::app::{add_to_router, App};

use super::{traits::RouterCtx, GlobalContext};

#[derive(Default)]
pub struct AppCtx {
    last_greet: Mutex<Option<String>>,
}

impl App for AppCtx {
    fn greet(&self, name: String) -> String {
        if let Ok(mut ctx) = self.last_greet.lock() {
            *ctx = Some(name.clone());
        }
        format!("Hello, {}! You've been greeted from Rust!", name)
    }

    fn last_greet(&self) -> Option<String> {
        if let Ok(ctx) = self.last_greet.try_lock() {
            return ctx.clone();
        }
        None
    }
}

impl RouterCtx<GlobalContext> for AppCtx {
    #[inline(always)]
    fn register_router(router: &mut Router<GlobalContext>) -> tauri_bindgen_host::Result<()> {
        add_to_router(router, |ctx| &ctx.app_ctx)
    }
}
