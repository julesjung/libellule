use std::error::Error;

use inquire::CustomType;
use pronote::Client;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let demo_url = Url::parse("https://demo.index-education.net/pronote/eleve.html").unwrap();
    let endpoint = CustomType::<Url>::new("Pronote URL:")
        .with_default(demo_url)
        .prompt()?;

    let client = Client::new(endpoint)?;
    let client = client.connect().await?;

    Ok(())
}
