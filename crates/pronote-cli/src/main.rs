use std::error::Error;

use inquire::{CustomType, Password, Text};
use pronote::client::Client;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let demo_url = Url::parse("https://demo.index-education.net/pronote/").unwrap();
    let instance_url = CustomType::<Url>::new("Pronote URL:")
        .with_default(demo_url)
        .prompt()?;

    let client = Client::from_url(instance_url).await?;
    let client = client.connect().await?;

    let username = Text::new("Username:").prompt()?;
    let password = Password::new("Password:").without_confirmation().prompt()?;

    let mut client = client.authenticate(&username, &password).await?;
    let user = client.user_information().await?;

    dbg!(user);

    Ok(())
}
