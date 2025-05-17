use std::fmt;

use strum_macros::EnumIter;

const START: &str = "start";
const STOP: &str = "stop";

#[derive(Debug, EnumIter)]
/// Actions to run on a game server
pub enum ServerAction {
    Start,
    Stop,
}

impl fmt::Display for ServerAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerAction::Start => write!(f, "{}", START),
            ServerAction::Stop => write!(f, "{}", STOP),
        }
    }
}

impl TryFrom<String> for ServerAction {
    type Error = std::io::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            START => Ok(Self::Start),
            STOP => Ok(Self::Stop),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid ServerCommand",
            )),
        }
    }
}
