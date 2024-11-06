use reqwest::Error;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::time::sleep;
use futures::future::join_all;

const RATE_LIMIT: usize = 5; // 5 requests per second
const API_URL: &str = "https://api.example.com/data"; // Example API URL

// The rate limiter controls access to the API requests
struct RateLimiter {
    semaphore: Semaphore,
    interval: Duration,
    last_request: Instant,
}

impl RateLimiter {
    fn new(rate_limit: usize) -> Self {
        RateLimiter {
            semaphore: Semaphore::new(rate_limit),
            interval: Duration::from_secs(1),
            last_request: Instant::now(),
        }
    }

    async fn acquire(&self) {
        // Wait for a permit to proceed with the request
        let permit = self.semaphore.acquire().await.unwrap();

        // Calculate the time between requests to stay within rate limit
        let elapsed = self.last_request.elapsed();
        if elapsed < self.interval {
            let sleep_duration = self.interval - elapsed;
            sleep(sleep_duration).await;
        }

        // Update the last request time and release the permit
        self.last_request = Instant::now();
        drop(permit);
    }

    async fn make_request(&self) -> Result<String, Error> {
        self.acquire().await;

        let response = reqwest::get(API_URL).await?;
        let body = response.text().await?;
        Ok(body)
    }
}

#[tokio::main]
async fn main() {
    let rate_limiter = RateLimiter::new(RATE_LIMIT);

    let mut handles = vec![];

    // Make concurrent API requests while respecting rate limit
    for _ in 0..10 {
        let limiter = rate_limiter.clone();
        handles.push(tokio::spawn(async move {
            match limiter.make_request().await {
                Ok(response) => println!("API response: {}", response),
                Err(err) => eprintln!("Error: {}", err),
            }
        }));
    }

    // Wait for all requests to finish
    join_all(handles).await;
}
