Timestamp and Random Number:

The app generates a unique input using the current timestamp (seconds since UNIX epoch) and a randomly generated number. This ensures that each request has a unique identifier.
Hash Generation:

The Sha256 struct from the sha2 crate is used to create a SHA-256 hash of the unique input string.
Hexadecimal Conversion:

The hash is converted to a hexadecimal string using the hex crate for easier readability.
Actix-web Setup:

We use actix-web to create an HTTP server and set up an endpoint (/generate_hash) that responds with the generated hash.