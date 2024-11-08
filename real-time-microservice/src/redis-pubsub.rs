use redis::AsyncCommands;
use std::env;
use tokio::task;

pub async fn broadcast_message(message: String) -> redis::RedisResult<()> {
    // Run in a separate task to avoid blocking the main thread
    task::spawn(async move {
        let redis_url = env::var("REDIS_URL").unwrap_or("redis://127.0.0.1/".to_string());
        let client = redis::Client::open(redis_url).expect("Invalid Redis URL");
        let mut conn = client.get_async_connection().await.expect("Failed to connect to Redis");

        conn.publish("chat_channel", message).await.expect("Failed to publish message");
    });

    Ok(())
}
