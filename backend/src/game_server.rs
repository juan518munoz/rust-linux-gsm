use once_cell::sync::Lazy;
use std::{env, fmt};
use strum_macros::EnumIter;

// Minecraft Server constants
static MCSERVER_CONTAINER_NAME: Lazy<String> = Lazy::new(|| {
    env::var("MCSERVER_CONTAINER_NAME").expect("MCSERVER_CONTAINER_NAME must be set in .env")
});
static MCSERVER_SERVICE_NAME: Lazy<String> = Lazy::new(|| {
    env::var("MCSERVER_SERVICE_NAME").expect("MCSERVER_SERVICE_NAME must be set in .env")
});

// Garry's Mod Server constants
static GMOD_CONTAINER_NAME: Lazy<String> =
    Lazy::new(|| env::var("GMOD_CONTAINER_NAME").expect("GMOD_CONTAINER_NAME must be set in .env"));
static GMOD_SERVICE_NAME: Lazy<String> =
    Lazy::new(|| env::var("GMOD_SERVICE_NAME").expect("GMOD_SERVICE_NAME must be set in .env"));

// Left 4 Dead 2 Server constants
static L4D2_CONTAINER_NAME: Lazy<String> =
    Lazy::new(|| env::var("L4D2_CONTAINER_NAME").expect("L4D2_CONTAINER_NAME must be set in .env"));
static L4D2_SERVICE_NAME: Lazy<String> =
    Lazy::new(|| env::var("L4D2_SERVICE_NAME").expect("L4D2_SERVICE_NAME must be set in .env"));

#[derive(Debug, EnumIter)]
pub(crate) enum GameServer {
    Minecraft,
    Gmod,
    L4d2,
}

impl fmt::Display for GameServer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameServer::Minecraft => write!(f, "{}", *MCSERVER_CONTAINER_NAME),
            GameServer::Gmod => write!(f, "{}", *GMOD_CONTAINER_NAME),
            GameServer::L4d2 => write!(f, "{}", *L4D2_CONTAINER_NAME),
        }
    }
}

impl TryFrom<String> for GameServer {
    type Error = std::io::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            s if s == MCSERVER_CONTAINER_NAME.as_str() => Ok(Self::Minecraft),
            s if s == GMOD_CONTAINER_NAME.as_str() => Ok(Self::Gmod),
            s if s == L4D2_CONTAINER_NAME.as_str() => Ok(Self::L4d2),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid GameServer",
            )),
        }
    }
}

impl GameServer {
    /// Returns the service name for the game server.
    pub(crate) fn service(&self) -> String {
        match self {
            GameServer::Minecraft => MCSERVER_SERVICE_NAME.to_string(),
            GameServer::Gmod => GMOD_SERVICE_NAME.to_string(),
            GameServer::L4d2 => L4D2_SERVICE_NAME.to_string(),
        }
    }
}
