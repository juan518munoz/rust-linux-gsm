use crate::components::Component;

pub fn stop_server_button(server: String) -> Component {
    format!(
        r#"
        <button
            class="btn btn-danger w-100"
            hx-post="/stop_server_clicked"
            hx-swap="outerHTML"
            hx-vals='{{ "server": "{server}" }}'
            onclick="this.innerText = '...'; this.disabled = true;">
            stop
        </button>
        "#,
        server = server
    )
}
