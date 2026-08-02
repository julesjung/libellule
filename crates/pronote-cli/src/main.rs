use std::error::Error;

use inquire::{CustomType, Password, Text};
use pronote::{client::Client, models::Tab};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let demo_url = Url::parse("https://demo.index-education.net/pronote/").unwrap();
    let instance_url = CustomType::<Url>::new("Pronote URL:")
        .with_default(demo_url)
        .prompt()?;

    let username = Text::new("Username:").prompt()?;
    let password = Password::new("Password:").without_confirmation().prompt()?;

    let client = Client::login(instance_url, username.as_str(), password.as_str()).await?;

    // let periods = user_parameters.tabs.periods.get(&Tab::Grades).unwrap();

    // dbg!(periods);

    // let default_periods = periods
    //     .periods
    //     .iter()
    //     .find(|period| period.id == periods.default)
    //     .unwrap();

    // let grades = client.get_grades(default_periods).await?;

    // dbg!(grades);

    Ok(())
}
