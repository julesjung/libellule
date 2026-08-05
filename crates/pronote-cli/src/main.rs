use std::error::Error;

use inquire::{CustomType, DateSelect, Password, Text};
use pronote::{Client, Instance};
use time::{Date, format_description::well_known::Iso8601};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let instance = {
        let demo_url = Url::parse("https://demo.index-education.net/pronote/").unwrap();
        let instance_url = CustomType::<Url>::new("Pronote URL:")
            .with_default(demo_url)
            .prompt()?;

        Instance::new(instance_url.to_string()).await?
    };

    let username = Text::new("Username:").prompt()?;
    let password = Password::new("Password:").without_confirmation().prompt()?;

    let mut client = Client::login(&instance, username.as_str(), password.as_str()).await?;

    // let periods = client.get_periods();

    // let default_period_id = client.get_default_period();
    // let default_period = periods
    //     .iter()
    //     .find(|period| period.id == default_period_id)
    //     .unwrap();

    // let grades = client.get_grades(default_period).await?;

    // dbg!(grades);

    let date = DateSelect::new("Date:").prompt()?;

    let date = date.format("%Y-%m-%d").to_string();

    let timetable = client
        .timetable(Date::parse(&date, &Iso8601::DATE)?)
        .await?;

    dbg!(timetable);

    Ok(())
}
