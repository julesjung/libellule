use libellule::error::Error;

#[derive(uniffi::Object)]
pub struct Instance {
    pub(crate) inner: libellule::Instance,
}

#[uniffi::export(async_runtime = "tokio")]
impl Instance {
    #[uniffi::constructor]
    pub async fn new(url: String) -> Result<Self, Error> {
        let instance = Instance {
            inner: libellule::Instance::new(url).await?,
        };

        Ok(instance)
    }

    pub fn label(&self) -> String {
        self.inner.label().to_string()
    }
}
