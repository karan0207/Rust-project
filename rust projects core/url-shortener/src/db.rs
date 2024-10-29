use rusqlite::{params, Connection, Result};
use std::time::{SystemTime, UNIX_EPOCH};

// Initializes the SQLite database
pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("url_shortener.db")?; // Opens a database file (creates if it doesn't exist)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls (
            id INTEGER PRIMARY KEY, 
            short_code TEXT NOT NULL UNIQUE, 
            original_url TEXT NOT NULL
        )",
        [],
    )?; // Creates the 'urls' table if it doesn't already exist
    Ok(conn)
}

// Inserts a URL into the database and returns the generated short code
pub fn insert_url(conn: &Connection, original_url: &str) -> Result<String> {
    let short_code = generate_short_code();
    conn.execute(
        "INSERT INTO urls (short_code, original_url) VALUES (?1, ?2)", 
        params![short_code, original_url],
    )?; // Inserts the short code and original URL into the 'urls' table
    Ok(short_code)
}

// Retrieves the original URL from the database using the short code
pub fn get_url(conn: &Connection, short_code: &str) -> Option<String> {
    let mut stmt = conn
        .prepare("SELECT original_url FROM urls WHERE short_code = ?1")
        .unwrap(); // Prepares the query to fetch the original URL based on the short code
    let result: Result<String> = stmt.query_row(params![short_code], |row| row.get(0)); // Executes the query and fetches the result

    result.ok() // Returns the original URL if found, or None if not found
}

// Generates a short code using the current timestamp
fn generate_short_code() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards"); // Calculates the time since UNIX epoch
    format!("{:x}", since_the_epoch.as_secs()) // Converts the time to a hexadecimal string (acts as the short code)
}
