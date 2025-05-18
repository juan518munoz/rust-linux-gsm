use crate::components::Component;

pub fn start_server_button(server: String) -> Component {
    format!(
        r#"
        <button
            class="btn btn-success w-100"
            hx-post="/start_server_clicked"
            hx-swap="outerHTML"
            hx-vals='{{ "server": "{server}" }}'
            hx-include="[name='api_token']"
            onclick="this.innerText = '...'; this.disabled = true;">
            start
        </button>
        "#,
        server = server
    )
}
