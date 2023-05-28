use tauri::{Builder, Runtime};

pub trait AppBuilder<R: Runtime> {
    fn build(builder: Builder<R>) -> Builder<R>;
}

pub trait AppBuilderNest<R: Runtime> {
    fn nest<I: AppBuilder<R>>(self) -> Self;
}

impl<R: Runtime> AppBuilderNest<R> for Builder<R> {
    #[inline(always)]
    fn nest<I: AppBuilder<R>>(self) -> Self {
        I::build(self)
    }
}
