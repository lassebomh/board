use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::Response,
    routing::get,
    Router,
};
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::broadcast::{self, Sender};
use tower_http::services::ServeDir;

struct AppState {
    db_pool: SqlitePool,
    games: DashMap<String, Sender<Message>>,
}

type SharedState = Arc<AppState>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let db_pool = SqlitePool::connect("./db.sqlite").await?;
    let games = DashMap::new();

    let state = Arc::new(AppState { db_pool, games });

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/ws/:game_id", get(get_ws_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_ws_handler(
    ws: WebSocketUpgrade,
    Path(game_id): Path<String>,
    State(state): State<SharedState>,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, game_id, state))
}

async fn handle_websocket(stream: WebSocket, game_id: String, state: SharedState) {
    // Split the WebSocket into send and receive parts
    let (mut sender, mut receiver) = stream.split();

    // Get or insert a broadcast channel sender for the specified game_id
    let tx = state
        .games
        .entry(game_id.clone())
        .or_insert_with(|| {
            // Initialize a broadcast channel with a capacity of 16
            broadcast::channel(16).0
        })
        .clone();

    // Clone tx to check receiver count after tasks complete
    let tx_clone = tx.clone();

    // Spawn a task to handle sending messages from the broadcast channel to this client
    let mut rx = tx.subscribe();
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Spawn a task to handle receiving messages from this client and broadcasting them
    let receive_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if tx.send(msg).is_err() {
                break;
            }
        }
    });

    // Wait for either the send or broadcast task to complete
    tokio::select! {
        _ = send_task => (),
        _ = receive_task => (),
    }

    // Cleanup: if no other receivers are left, remove the game_id entry from DashMap
    if tx_clone.receiver_count() == 0 {
        state.games.remove(&game_id);
    }
}

#[derive(Serialize, Deserialize)]
enum WebSocketCommand {
    Move {
        game_id: String,
        piece_id: String,
        x: f64,
        y: f64,
    },
}
