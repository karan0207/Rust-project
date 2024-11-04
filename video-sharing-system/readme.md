# P2P Video Sharing System

This project implements a simple peer-to-peer (P2P) video sharing system in Rust. The application allows users to send video files directly to each other without relying on a central server, leveraging Rust's performance and safety features.

## Features

- **Peer-to-Peer File Transfer**: Send video files directly between users.
- **Asynchronous Networking**: Utilizes `tokio` for handling multiple connections concurrently.
- **Easy to Use CLI**: Simple command-line interface for sending files.

## Requirements

- Rust (1.54.0 or later)
- Cargo (comes with Rust)

## Installation

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/yourusername/p2p_video_sharing.git
   cd p2p_video_sharing
