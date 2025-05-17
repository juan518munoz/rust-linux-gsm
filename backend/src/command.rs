use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::{GameServer, action::ServerAction};

pub(crate) fn build_command(game_server: GameServer, action: ServerAction) -> Command {
    let game_servers_dir = PathBuf::from("../game-servers");

    let mut cmd = Command::new("make");
    cmd.current_dir(game_servers_dir);

    match action {
        ServerAction::Start => {
            cmd.arg("start");
            cmd.arg(format!("SERVICE={}", game_server.service()));
        }
        ServerAction::Stop => {
            cmd.arg("stop");
            cmd.arg(format!("SERVER={}", game_server));
        }
    }

    cmd.arg(action.to_string())
        .arg(format!("{}", game_server))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    cmd
}

pub(crate) async fn run_command(mut command: Command) -> String {
    let mut child = command.spawn().expect("Failed to start command");
    match child.wait() {
        Ok(_) => "Command ran succesfully".to_string(),
        Err(e) => format!("Failed to run command: {}", e),
    }
}
