use std::collections::HashMap;
use reqwest::Client;
use select::document::Document;
use select::predicate::Name;
use tokio::task;
use tokio::time::{sleep, Duration};

// Struct to hold scraped data
struct ScrapedData {
    url: String,
    content: String,
}

// Function to scrape a single URL
async fn scrape_url(client: &Client, url: &str) -> Option<ScrapedData> {
    match client.get(url).send().await {
        Ok(response) => {
            if let Ok(text) = response.text().await {
                let document = Document::from(text.as_str());
                let mut content = String::new();

                // Extract <p> tag content for simplicity
                for node in document.find(Name("p")) {
                    content.push_str(&node.text());
                }

                Some(ScrapedData {
                    url: url.to_string(),
                    content,
                })
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

// Function to concurrently scrape multiple URLs
async fn scrape_sites(client: &Client, urls: Vec<&str>) -> HashMap<String, String> {
    let mut tasks = Vec::new();

    for url in urls {
        let client = client.clone();
        tasks.push(task::spawn(async move {
            if let Some(data) = scrape_url(&client, url).await {
                Some((data.url, data.content))
            } else {
                None
            }
        }));
    }

    let mut results = HashMap::new();
    for task in tasks {
        if let Ok(Some((url, content))) = task.await {
            results.insert(url, content);
        }
    }
    results
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    // Example URLs to scrape
    let urls = vec![
        "https://www.example.com",
        "https://www.rust-lang.org",
    ];

    println!("Starting to scrape websites...");

    // Scrape the sites concurrently
    let scraped_data = scrape_sites(&client, urls).await;

    // Simulate data processing (e.g., filtering specific content)
    for (url, content) in scraped_data {
        println!("Scraped from: {}\n", url);
        println!("Content snippet: {:.100}...\n", content); // Printing first 100 chars
        sleep(Duration::from_secs(1)).await; // Simulating processing delay
    }
}
