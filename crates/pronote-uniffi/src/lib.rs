use std::sync::Arc;

use pronote::client::Client as PronoteClient;
use pronote::error::Error;
use pronote::instance::Instance;
use pronote::models::{GradesData, Period};
use tokio::sync::Mutex;

uniffi::setup_scaffolding!();

#[derive(uniffi::Object)]
pub struct Client {
    inner: Mutex<PronoteClient>,
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    #[uniffi::constructor]
    pub async fn new(
        instance: Arc<Instance>,
        password: &str,
        username: &str,
    ) -> Result<Client, Error> {
        let client = PronoteClient::login((*instance).clone(), password, username).await?;

        Ok(Client {
            inner: Mutex::new(client),
        })
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl Client {
    pub async fn get_periods(&self) -> Vec<Period> {
        self.inner.lock().await.get_periods()
    }

    pub async fn get_default_period(&self) -> String {
        self.inner.lock().await.get_default_period()
    }

    pub async fn get_grades(&self, period: &Period) -> Result<GradesData, Error> {
        self.inner.lock().await.get_grades(period).await
    }
}
