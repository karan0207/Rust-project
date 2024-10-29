use warp::Filter;
use std::convert::Infallible;
use std::path::PathBuf;
use tokio::fs::File;
use tokio_util::codec::{FramedRead, BytesCodec};
use futures::StreamExt;

#[tokio::main]
async fn main() {
    // Route for streaming video
    let stream_route = warp::path!("stream" / String)
        .and_then(stream_video);

    // Start the warp server
    warp::serve(stream_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// Function to stream video from a file
async fn stream_video(file_name: String) -> Result<impl warp::Reply, Infallible> {
    // Specify the path to your video file
    let path = PathBuf::from(format!("./videos/{}", file_name));

    // Open the file
    let file = File::open(path).await;
    
    match file {
        Ok(f) => {
            let stream = FramedRead::new(f, BytesCodec::new());
            // Return the streaming response
            let response = warp::stream::iter(stream).map(|bytes| Ok::<_, Infallible>(bytes));
            Ok(warp::reply::stream(response))
        },
        Err(_) => {
            // Return 404 if file not found
            Ok(warp::reply::with_status("File not found", warp::http::StatusCode::NOT_FOUND))
        }
    }
}
