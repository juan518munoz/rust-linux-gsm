use std::env;

use crate::GameServer;
use crate::{
    action::ServerAction,
    command::{build_command, run_command},
};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;

pub async fn server_action(
    Path((server_name, action)): Path<(String, String)>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> impl IntoResponse {
    log::info!(
        "Received request for server: {} with action: {}",
        server_name,
        action
    );

    let expected_bearer = match env::var("AUTH_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            log::error!("AUTH_TOKEN not set in .env");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error\n");
        }
    };

    if bearer.token() != expected_bearer {
        log::debug!("Unauthorized: Invalid API token");
        return (StatusCode::UNAUTHORIZED, "Invalid Token\n");
    }

    let game_server = match GameServer::try_from(server_name.clone()) {
        Ok(server) => server,
        Err(_) => {
            log::error!("Failed to parse game server name: {}", server_name);
            return (StatusCode::BAD_REQUEST, "Invalid Server\n");
        }
    };

    let action = match ServerAction::try_from(action.clone()) {
        Ok(act) => act,
        Err(_) => {
            log::error!("Failed to parse action: {}", action);
            return (StatusCode::BAD_REQUEST, "Invalid Action\n");
        }
    };

    let command = build_command(game_server, action);
    let _output = run_command(command).await;
    (StatusCode::OK, "OK")
}
