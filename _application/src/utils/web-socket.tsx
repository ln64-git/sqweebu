import { useState, useEffect } from "react";

interface WebSocketData {
  receivedMessage: string | null;
  sendMessage: (message: string) => void;
}

const useWebSocket = (url: string): WebSocketData => {
  const [ws, setWs] = useState<WebSocket | null>(null);
  const [receivedMessage, setReceivedMessage] = useState<string | null>(null);

  useEffect(() => {
    const socket = new WebSocket(url);
    socket.onopen = () => {
      console.log("WebSocket connection established.");
      setWs(socket);
    };
    socket.onmessage = (event) => {
      console.log("Received message from server:", event.data);
      setReceivedMessage(event.data);
    };
    socket.onclose = () => {
      console.log("WebSocket connection closed.");
    };
    return () => {
      if (socket) {
        socket.close();
      }
    };
  }, [url]);

  const sendMessage = (message: string) => {
    if (ws) {
      console.log(message);
      ws.send(message);
    }
  };

  return { receivedMessage, sendMessage };
};

export default useWebSocket;
