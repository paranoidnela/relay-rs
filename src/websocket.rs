use std::net::{TcpListener, TcpStream, IpAddr};
use tungstenite::accept;
use tungstenite::protocol::Message;

use crate::actions::{add_time, subtract_time, reset_lane, reset_all, status_json, success_json};

fn handle_connection(stream: TcpStream) {
    let mut ws_stream = accept(stream).expect("Error during WebSocket handshake");

    loop {
        let msg = ws_stream.read().expect("Error reading message");

        match msg {            
            Message::Text(text) => {
                let response_message = String::new();

                if text.starts_with("/add_time/") {
                    let response_message = text.trim_start_matches("/add_time/").trim();
                    let lane = response_message.parse().expect("Not a valid number");
                    add_time(lane);
                    let response = success_json();
                    ws_stream.send(Message::Text(response.into())).expect("Error sending message");

                } else if text.starts_with("/subtract_time/") {
                    let response_message = text.trim_start_matches("/subtract_time/").trim();
                    let lane = response_message.parse().expect("Not a valid number");
                    subtract_time(lane);
                    let response = success_json();
                    ws_stream.send(Message::Text(response.into())).expect("Error sending message");

                } else if text.starts_with("/reset_lane/") {
                    let response_message = text.trim_start_matches("/reset_lane/").trim();
                    let lane = response_message.parse().expect("Not a valid number");
                    reset_lane(lane);
                    let response = success_json();
                    ws_stream.send(Message::Text(response.into())).expect("Error sending message");

                } else if text.starts_with("/reset_all") {
                    reset_all();
                    let response = success_json();
                    ws_stream.send(Message::Text(response.into())).expect("Error sending message");

                } else if text.starts_with("/status") {
                    let response = status_json();
                    ws_stream.send(Message::Text(response.into())).expect("Error sending message");
                }
            }
            Message::Close(_) => {
                break;
            }
            _ => {}
        }
    }
}

fn is_localhost(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(addr) => addr.is_loopback(),
        IpAddr::V6(addr) => addr.is_loopback(),
    }
}

pub fn websocket_server() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).expect("Failed to bind");

    println!("WebSocket server listening on {}", addr);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peer_addr = stream.peer_addr().expect("Could not get peer address");
                println!("New connection: {:?}", peer_addr);
                if is_localhost(&peer_addr.ip()) {
                    handle_connection(stream);
                }
                else {
                    println!("Connection attempt from non-localhost: {}", peer_addr);
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
