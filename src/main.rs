use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{string::String, vec::Vec};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    pub chain_id: u8,
    pub validators: Vec<String>,
}

impl Config {
    pub fn new(chain_id: u8, validators: Vec<String>) -> Self {
        Self {
            chain_id,
            validators,
        }
    }
}

fn main() -> Result<()> {
    let f = std::fs::File::create("./test.yaml")?;
    let a = Config::new(
        4,
        vec![
            "/ip4/127.0.0.1/tcp/8000".to_string(),
            "/ip4/127.0.0.1/tcp/8010".to_string(),
            "/ip4/127.0.0.1/tcp/8020".to_string(),
            "/ip4/127.0.0.1/tcp/8030".to_string(),
        ],
    );

    serde_yaml::to_writer(f, &a)?;
    //println!("{}", s);

    let f = std::fs::File::open("test.yaml")?;
    let d: Config = serde_yaml::from_reader(f)?;
    println!("{:?}", d);

    Ok(())
}
