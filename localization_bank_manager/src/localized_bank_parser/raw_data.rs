use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

pub type RawData = HashMap<Box<str>, RawRecord>;

#[derive(Serialize, Deserialize, Debug)]
pub struct RawRecord {
    pub text: HashMap<Box<str>, HashMap<Box<str>, Vec<Value>>>,
    pub tag: Option<Box<[Box<str>]>>,
}

impl RawRecord {
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<RawData> {
        let f = std::fs::File::open(path)?;
        Ok(serde_yaml::from_reader(f)?)
    }
}
