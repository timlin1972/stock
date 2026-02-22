use std::fmt;
use std::fs;

use serde::{Deserialize, Serialize};

const CFG_FILE: &str = "cfg.json";

#[derive(Serialize, Deserialize)]
pub struct CfgData {
    pub fugle_api_key: String,
}

impl fmt::Debug for CfgData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CfgData")
            .field("fugle_api_key", &"****")
            .finish()
    }
}

impl CfgData {
    pub fn new() -> Self {
        let file = fs::File::open(CFG_FILE).unwrap();
        let reader = std::io::BufReader::new(file);
        serde_json::from_reader::<_, CfgData>(reader).unwrap()
    }
}
