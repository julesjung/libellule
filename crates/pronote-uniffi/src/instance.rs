use crate::error::Error;

#[derive(uniffi::Object)]
pub struct Instance {
    pub(crate) inner: pronote::Instance,
}

#[uniffi::export(async_runtime = "tokio")]
impl Instance {
    #[uniffi::constructor]
    pub async fn new(url: String) -> Result<Self, Error> {
        let instance = Instance {
            inner: pronote::Instance::new(url).await?,
        };

        Ok(instance)
    }
}
