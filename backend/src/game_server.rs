use std::{
    env, fmt,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
};

use strum_macros::EnumIter;

const PROJECT_ZOMBOID_SV_NAME: &'static str = "pzserver";
const GMOD_SV_NAME: &'static str = "gmodserver";

#[derive(Debug, EnumIter)]
pub enum GameServer {
    PZomboid,
    Gmod,
}

impl fmt::Display for GameServer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameServer::PZomboid => write!(f, "{}", PROJECT_ZOMBOID_SV_NAME),
            GameServer::Gmod => write!(f, "{}", GMOD_SV_NAME),
        }
    }
}

impl TryFrom<String> for GameServer {
    type Error = std::io::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            PROJECT_ZOMBOID_SV_NAME => Ok(Self::PZomboid),
            GMOD_SV_NAME => Ok(Self::Gmod),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid GameServer",
            )),
        }
    }
}

const MONITOR: &'static str = "monitor";
const START: &'static str = "start";
const STOP: &'static str = "stop";
const RESTART: &'static str = "restart";

#[derive(Debug, EnumIter)]
pub enum ServerAction {
    Monitor,
    Start,
    Stop,
    Restart,
}

impl fmt::Display for ServerAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerAction::Monitor => write!(f, "{}", MONITOR),
            ServerAction::Start => write!(f, "{}", START),
            ServerAction::Stop => write!(f, "{}", STOP),
            ServerAction::Restart => write!(f, "{}", RESTART),
        }
    }
}

impl TryFrom<String> for ServerAction {
    type Error = std::io::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            MONITOR => Ok(Self::Monitor),
            START => Ok(Self::Start),
            STOP => Ok(Self::Stop),
            RESTART => Ok(Self::Restart),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid ServerCommand",
            )),
        }
    }
}

pub(crate) fn build_command(game_server: GameServer, action: ServerAction) -> Command {
    // TODO: Update backend from here
    let server_dir = env::var("HOME").unwrap();
    let mut server_bin_path = PathBuf::from(server_dir);
    server_bin_path.push(game_server.to_string()); // We push game_server twice because convention is
    server_bin_path.push(game_server.to_string()); // home/users/game-server/<game_server>/<game_server>

    let mut cmd = Command::new(server_bin_path);
    cmd.arg(action.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    cmd
}

pub(crate) async fn run_command(mut command: Command) -> String {
    let mut child = command.spawn().expect("Failed to start command");
    let stdout = BufReader::new(child.stdout.take().expect("Failed to capture stdout"));

    let mut output_string = String::new();
    for line in stdout.lines() {
        output_string.push_str(&line.unwrap());
        output_string.push('\n');
    }

    output_string
}
