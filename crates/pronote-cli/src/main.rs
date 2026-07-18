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
    let (client, parameters) = client.connect().await?;

    println!(
        "Found instance \"{}\" using PRONOTE version {}",
        parameters.general.name, parameters.general.version
    );

    let username = Text::new("Username:").prompt()?;
    let password = Password::new("Password:").prompt()?;

    let (_client, fullname) = client.authenticate(&username, &password).await?;

    println!("User: {}", fullname);

    Ok(())
}
