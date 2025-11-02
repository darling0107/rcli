use std::fs;

use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    // The from_path function will return data of The Result type
    // It's advisable to avoid using unwrap() in development environments.
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    // let records = reader
    //     .deserialize()
    //     .map(|record| record.unwrap())
    //     .collect::<Vec<Player>>();
    // println!("{:?}", records)
    for result in reader.deserialize() {
        let record: Player = result?;
        // println!("{:?}", record);
        ret.push(record);
    }

    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;

    Ok(())
}
