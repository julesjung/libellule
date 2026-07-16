use url::Url;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

pub struct Client<S = Disconnected> {
    endpoint: Url,
    http: reqwest::Client,
    state: S,
}

pub struct Disconnected;

pub struct Connected {
    session_id: u8,
}

impl Client {
    pub fn new(endpoint: Url) -> Result<Client, reqwest::Error> {
        let http = reqwest::Client::builder().user_agent(USER_AGENT).build()?;

        Ok(Client {
            endpoint,
            http,
            state: Disconnected,
        })
    }
}

impl Client<Disconnected> {
    pub async fn connect(self) -> Result<Client<Connected>, reqwest::Error> {
        self.http
            .get(self.endpoint.clone())
            .send()
            .await?
            .text()
            .await?;

        Ok(Client {
            endpoint: self.endpoint,
            http: self.http,
            state: Connected { session_id: 0 },
        })
    }
}
