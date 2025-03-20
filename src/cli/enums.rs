use std::str::FromStr;

use anyhow::anyhow;

#[derive(Clone, Debug)]
pub enum Mode {
    Client,
    Server,
}
impl FromStr for Mode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s.to_lowercase().as_str() {
            "client" => Ok(Mode::Client),
            "server" => Ok(Mode::Server),
            _ => Err(anyhow!("Invalid mode")),
        }
    }
}
impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::Client => "client".to_string(),
            Mode::Server => "server".to_string(),
        }
    }
}
