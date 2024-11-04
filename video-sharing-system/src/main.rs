use std::io::{self, Write};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

mod peer;
mod video;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<String>(32);

    // Start UDP listener
    let socket = UdpSocket::bind("0.0.0.0:8080").await.unwrap();
    let socket_clone = socket.try_clone().unwrap();

    tokio::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            let (len, addr) = socket.recv_from(&mut buf).await.unwrap();
            let msg = String::from_utf8_lossy(&buf[..len]);
            println!("Received from {}: {}", addr, msg);
            // Process the message or pass it to the sender
            tx.send(msg.to_string()).await.unwrap();
        }
    });

    loop {
        // Display menu
        println!("Enter command: (send <peer_ip> <file_path> or exit)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if trimmed == "exit" {
            break;
        } else {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() == 3 && parts[0] == "send" {
                let peer_ip = parts[1];
                let file_path = parts[2];
                // Send the video file to the peer
                video::send_video(&socket_clone, peer_ip, file_path).await;
            }
        }
    }
}
