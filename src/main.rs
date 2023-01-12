/**************************************
 * DESEALIZE TOML USING CUSTOM STRUCTS
 **************************************/
use serde::Deserialize;

use std::collections::HashMap;
use toml::de::Error;

fn main() -> Result<(), Error> {
    #[derive(Debug, Deserialize)]
    struct Package {
        name: String,
        version: String,
        authors: Vec<String>,
    }
    #[derive(Debug, Deserialize)]
    struct Config {
        package: Package,
        dependencies: HashMap<String, String>,
    }

    let toml = r#"
    [package]
    name = "danger_zone"
    version = "0.1.0"
    authors = ["Archer Sterling <sterlinarcher@gof****self.org>"]

    [dependencies]
    serde = "1.0"
    "#;

    let package_info: Config = toml::from_str(toml)?;

    println!("{:#?}", package_info);
    Ok(())
}
