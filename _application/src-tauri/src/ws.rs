use ws::{listen, CloseCode, Handler, Message, Result};

struct WebSocketServer;

impl Handler for WebSocketServer {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Received message: {:?}", msg);
        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing: {:?}, {:?}", code, reason);
    }
}

pub fn start_websocket_server() {
    println!("Starting WebSocket server...");
    listen("127.0.0.1:8080", |_out| WebSocketServer {}).unwrap();
}
