use std::fs;
use std::io::Error;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;

#[derive(Debug, Deserialize, Serialize)]
pub enum MarkStyle {
    Default,
    Extended,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub mark_style: MarkStyle,
    pub editor: String,
}

lazy_static! {
    pub static ref CONFIG: Mutex<Option<Config>> = Mutex::new(None);
}

pub fn initialize_config() -> Result<(), Error> {
    let _config = fs::read_to_string("./src/config.json")?;
    let config: Config = serde_json::from_str(&_config)?;

    let mut guard = CONFIG.lock().unwrap();
    assert!( guard.is_none() );

    *guard = Some(config);
    Ok(())
}
