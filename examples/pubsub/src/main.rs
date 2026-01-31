use futures::StreamExt;
use nimiq_jsonrpc_client::websocket::WebsocketClient;
use nimiq_jsonrpc_client::Client; // Import the Client trait
use serde_json::Value;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Connecting to websocket");

    let url = "ws://localhost:25558".parse().unwrap();
    let token = Some("lHI9Qhtb5XQ18XWvbO41GePxaHB0PbRdtI5wBzYH".to_string());

    let client = match WebsocketClient::new(url, token).await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to connect websocket: {:?}", e);
            return;
        }
    };

    let status: Value = client
        .send_request::<(), Value>("minecraft:server/status", None)
        .await
        .expect("Failed to send subscription request");

    log::info!("Received status response: {}", serde_json::to_string(&status).unwrap());
}

#[derive(serde::Deserialize, Debug)]
struct ServerManagementVersion {
    protocol: i32,
    name: String,
}

#[derive(serde::Deserialize, Debug)]
struct ServerManagementPlayer {
    name: String,
    id: String,
}

#[derive(serde::Deserialize, Debug)]
struct ServerManagementStateResponse {
    players: Vec<ServerManagementPlayer>,
    started: bool,
    version: ServerManagementVersion,
}
