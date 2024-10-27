use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::{Html, Response},
    routing::get,
    Router,
};
use sqlx::SqlitePool;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect("./db.sqlite").await?;

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(get_index_handler))
        .route("/ws", get(get_ws_handler))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_index_handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn get_ws_handler(ws: WebSocketUpgrade, State(pool): State<SqlitePool>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, pool))
}

async fn handle_socket(mut ws: WebSocket, pool: SqlitePool) {
    ws.send(Message::Text("hello".to_string())).await;

    if let Some(Ok(Message::Text(msg))) = ws.recv().await {
        println!("{:}", msg);
    }
}
