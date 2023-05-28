enum RouterBuilderStatus {
    Ok,
    Err(tauri_bindgen_host::anyhow::Error),
}
use tauri_bindgen_host::ipc_router_wip::Router;

use super::traits::RouterCtx;

pub struct RouterBuilder<U> {
    status: RouterBuilderStatus,
    inner: Router<U>,
}
impl<U> RouterBuilder<U> {
    #[inline(always)]
    pub fn new(data: U) -> Self {
        Self {
            status: RouterBuilderStatus::Ok,
            inner: Router::new(data),
        }
    }

    #[inline(always)]
    pub fn nest<C: RouterCtx<U>>(mut self) -> Self {
        match &self.status {
            RouterBuilderStatus::Ok => {
                if let Err(e) = C::register_router(&mut self.inner) {
                    self.status = RouterBuilderStatus::Err(e)
                }
            }
            RouterBuilderStatus::Err(_) => {}
        }
        self
    }

    #[inline(always)]
    pub fn build(self) -> tauri_bindgen_host::Result<Router<U>> {
        match self.status {
            RouterBuilderStatus::Ok => Ok(self.inner),
            RouterBuilderStatus::Err(e) => Err(e),
        }
    }
}
