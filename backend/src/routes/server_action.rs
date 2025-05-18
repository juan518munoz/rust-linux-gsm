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
    let expected_bearer = env::var("AUTH_TOKEN").expect("AUTH_TOKEN must be set in .env");

    if bearer.token() != expected_bearer {
        return (StatusCode::UNAUTHORIZED, "Invalid Token\n");
    }

    let game_server = match GameServer::try_from(server_name) {
        Ok(server) => server,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid Server\n"),
    };

    let action = match ServerAction::try_from(action) {
        Ok(act) => act,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid Action\n"),
    };

    let command = build_command(game_server, action);
    let _output = run_command(command).await;
    (StatusCode::OK, "OK")
}
