#[allow(clippy::all)]
#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod app {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait App: Sized {
        fn greet(&mut self, name: String) -> String;
        fn last_greet(&mut self) -> Option<String>;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: App + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "app",
                "greet",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    name: String,
                | -> ::tauri_bindgen_host::anyhow::Result<String> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.greet(name))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "app",
                "last_greet",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Option<String>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.last_greet())
                },
            )?;
        Ok(())
    }
}
