#[macro_export]
macro_rules! generate_methods {
    ($($method:ident),+) => {
        paste! {
            $(
                #[allow(unused)]
                pub async fn $method<T: for<'de> serde::Deserialize<'de>>(
                    &mut self,
                    route: &str,
                    body: Option<&impl Serialize>,
                ) -> Result<(Response<T>), Error> {
                    self.act::<T>(Method::[<$method:upper>], route, body).await
                }
            )+
        }
    };
}
