use std::sync::Arc;
use surrealdb::Surreal;
use tokio::sync::Mutex;
use ws::{listen, CloseCode, Handler, Message, Result};

struct WebSocketServer {
    db_lock: Arc<Mutex<Surreal<surrealdb::engine::local::Db>>>,
}

impl Handler for WebSocketServer {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let db = self.db_lock.clone(); // Clone the Arc to capture in the closure
        println!("{:#?}", msg);
        println!("This is called on first execution.",);
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing: {:?}, {:?}", code, reason);
    }
}

pub async fn start_websocket_server(db_lock: Arc<Mutex<Surreal<surrealdb::engine::local::Db>>>) {
    println!("Starting WebSocket server...");
    listen("127.0.0.1:8080", move |_out| WebSocketServer {
        db_lock: db_lock.clone(),
    })
    .unwrap();
}
