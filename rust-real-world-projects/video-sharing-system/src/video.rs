use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::UdpSocket;

pub async fn send_video(socket: &UdpSocket, peer_ip: &str, file_path: &str) {
    let mut file = File::open(file_path).await.expect("Unable to open file");
    let mut buffer = vec![0; 1024];

    loop {
        let bytes_read = file.read(&mut buffer).await.expect("Failed to read file");
        if bytes_read == 0 {
            break; // End of file
        }
        // Send data to the peer
        let _ = socket.send_to(&buffer[..bytes_read], peer_ip).await;
    }
    println!("Video sent to {}", peer_ip);
}
