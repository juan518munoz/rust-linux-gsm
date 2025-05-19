use once_cell::sync::Lazy;
use std::{env, fmt, process::Command};
use strum_macros::EnumIter;

// Minecraft Server constants
static MCSERVER_CONTAINER_NAME: Lazy<String> = Lazy::new(|| {
    env::var("MCSERVER_CONTAINER_NAME").expect("MCSERVER_CONTAINER_NAME must be set in .env")
});
static MCSERVER_SERVICE_NAME: Lazy<String> = Lazy::new(|| {
    env::var("MCSERVER_SERVICE_NAME").expect("MCSERVER_SERVICE_NAME must be set in .env")
});
static MCSERVER_PORT: Lazy<String> =
    Lazy::new(|| env::var("MCSERVER_PORT").expect("MCSERVER_PORT must be set in .env"));

// Garry's Mod Server constants
static GMOD_CONTAINER_NAME: Lazy<String> =
    Lazy::new(|| env::var("GMOD_CONTAINER_NAME").expect("GMOD_CONTAINER_NAME must be set in .env"));
static GMOD_SERVICE_NAME: Lazy<String> =
    Lazy::new(|| env::var("GMOD_SERVICE_NAME").expect("GMOD_SERVICE_NAME must be set in .env"));
static GMOD_PORT: Lazy<String> =
    Lazy::new(|| env::var("GMOD_PORT").expect("GMOD_PORT must be set in .env"));

// Left 4 Dead 2 Server constants
static L4D2_CONTAINER_NAME: Lazy<String> =
    Lazy::new(|| env::var("L4D2_CONTAINER_NAME").expect("L4D2_CONTAINER_NAME must be set in .env"));
static L4D2_SERVICE_NAME: Lazy<String> =
    Lazy::new(|| env::var("L4D2_SERVICE_NAME").expect("L4D2_SERVICE_NAME must be set in .env"));
static L4D2_PORT: Lazy<String> =
    Lazy::new(|| env::var("L4D2_PORT").expect("L4D2_PORT must be set in .env"));

#[derive(Debug, EnumIter)]
/// Available game servers to run actions on
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

    /// Returns the endpoint for the game server.
    pub(crate) fn endpoint(&self) -> String {
        let public_ip = match env::var("PUBLIC_IP") {
            Ok(ip) => ip,
            Err(_) => {
                log::error!("PUBLIC_IP not set in .env");
                return "PUBLIC_IP not set".to_string();
            }
        };

        let port = match self {
            GameServer::Minecraft => MCSERVER_PORT.to_string(),
            GameServer::Gmod => GMOD_PORT.to_string(),
            GameServer::L4d2 => L4D2_PORT.to_string(),
        };
        format!("{}:{}", public_ip, port)
    }

    /// Checks if the game server is running.
    pub(crate) fn is_running(&self) -> bool {
        let container_name = self.to_string();

        let output = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "docker ps --filter name=^/{}$ --filter status=running --format '{{{{.Names}}}}' | grep -q '^{}$'",
                container_name, container_name
            ))
            .output();

        match output {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }
}
