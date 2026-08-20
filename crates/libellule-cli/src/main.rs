use std::error::Error;
use std::fmt::Display;

use inquire::{CustomType, DateSelect, Password, Select, Text};
use libellule::{Client, Instance};
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

    let client = Client::login(&instance, username.as_str(), password.as_str()).await?;

    loop {
        let commands = vec![
            Command::Timetable,
            Command::Grades,
            Command::Menu,
            Command::Homework,
            Command::Quit,
        ];
        let command: Command = Select::new("Command:", commands).prompt()?;

        match command {
            Command::Timetable => timetable(&client).await?,
            Command::Grades => grades(&client).await?,
            Command::Menu => menu(&client).await?,
            Command::Homework => homework(&client).await?,
            Command::Quit => break,
        }
    }

    Ok(())
}

async fn timetable(client: &Client) -> Result<(), Box<dyn Error>> {
    let date = DateSelect::new("Date:").prompt()?;

    let date = date.format("%Y-%m-%d").to_string();

    let timetable = client
        .timetable(Date::parse(&date, &Iso8601::DATE)?)
        .await?;

    dbg!(timetable);

    Ok(())
}

async fn grades(client: &Client) -> Result<(), Box<dyn Error>> {
    let periods = client.periods();

    let default_period_id = client.default_period();
    let default_period = periods
        .iter()
        .find(|period| period.id == default_period_id)
        .unwrap();

    let grades = client.grades(default_period).await?;

    dbg!(grades);

    Ok(())
}

async fn menu(client: &Client) -> Result<(), Box<dyn Error>> {
    let date = DateSelect::new("Date:").prompt()?;

    let date = date.format("%Y-%m-%d").to_string();

    let menu = client.menu(Date::parse(&date, &Iso8601::DATE)?).await?;

    dbg!(menu);

    Ok(())
}

async fn homework(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let date = DateSelect::new("Date:").prompt()?;
    let date = date.format("%Y-%m-%d").to_string();

    let homework = client.homework(Date::parse(&date, &Iso8601::DATE)?).await?;

    dbg!(homework);

    Ok(())
}

enum Command {
    Timetable,
    Grades,
    Menu,
    Homework,
    Quit,
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Timetable => write!(f, "Timetable"),
            Command::Grades => write!(f, "Grades"),
            Command::Menu => write!(f, "Menu"),
            Command::Homework => write!(f, "Homework"),
            Command::Quit => write!(f, "Quit"),
        }
    }
}
