use warp::Filter;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // Directory where files will be stored
    let file_store = Arc::new(Mutex::new("./sync_directory".to_string()));

    // Define the route for file upload
    let upload_route = warp::path!("upload" / String)
        .and(warp::body::bytes())
        .and(with_store(file_store.clone()))
        .and_then(upload_file);

    warp::serve(upload_route).run(([127, 0, 0, 1], 8080)).await;
}

// Extract the file store from the warp filter
fn with_store(
    file_store: Arc<Mutex<String>>,
) -> impl Filter<Extract = (Arc<Mutex<String>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || file_store.clone())
}

// Handler function for file upload
async fn upload_file(filename: String, data: bytes::Bytes, file_store: Arc<Mutex<String>>) -> Result<impl warp::Reply, warp::Rejection> {
    let store_dir = file_store.lock().await;
    let file_path = format!("{}/{}", *store_dir, filename);

    let mut file = OpenOptions::new().create(true).write(true).open(&file_path)
        .map_err(|_| warp::reject::custom("Error creating file"))?;
    
    file.write_all(&data).map_err(|_| warp::reject::custom("Error writing to file"))?;

    Ok(warp::reply::with_status("File uploaded successfully", warp::http::StatusCode::OK))
}
