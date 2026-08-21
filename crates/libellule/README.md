# Libellule

[![Version](https://img.shields.io/crates/v/libellule.svg)](https://crates.io/crates/libellule)
[![Documentation](https://docs.rs/libellule/badge.svg)](https://docs.rs/libellule)
[![CI](https://img.shields.io/github/actions/workflow/status/julesjung/libellule/ci.yml)](https://github.com/julesjung/libellule/actions/workflows/ci.yml)
![License](https://img.shields.io/crates/l/libellule.svg)

A Rust implementation of the PRONOTE protocol.

> [!WARNING]
> Libellule is not affiliated with, endorsed by, or sponsored by INDEX ÉDUCATION.

## Usage

Add libellule as a dependency in your Cargo.toml:

```toml
[dependencies]
libellule = "0.1"
tokio = { version = "1", features = ["full"] }
```

And here is a usage example to get you started:

```rust
use libellule::{Client, Instance};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance_url = String::from("https://demo.index-education.net/pronote/");
    let instance = Instance::new(instance_url).await?;
    let username = "demonstration";
    let password = "pronotevs";

    let client = Client::login(&instance, username, password).await?;

    let periods = client.periods();
    let default_period_id = client.default_period();
    let default_period = periods
        .iter()
        .find(|period| period.id == default_period_id)
        .unwrap();

    let grades = client.grades(default_period).await?;

    println!("{:#?}", grades);

    Ok(())
}
```

This code displays all the grades for the default period from PRONOTE's demo instance.

## License

Released under the GNU General Public License v3.0.
