use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "8080";

const ROOT_DIRECTORY: &str = "./webserver/";


fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Sets max request buffer size to 1024.
    
    // Read the request from the client.
    stream.read(&mut buffer).unwrap(); // Reads incoming bytes and passes it into the buffer.
    let request = String::from_utf8_lossy(&buffer[..]); // Decodes the bytes into UTF-8.
    println!("Received request:\n{}", request);

    // Extract requested path.
    let requested_path = parse_http_request(&request);
    println!("Extracted Requested Path: {}\n", requested_path);

    // Get the full path based on the default root directory.
    let full_path = format!("{}/{}", ROOT_DIRECTORY, requested_path);

    // If the requested path is a directory then return the index.html for that directory.
    let full_path = if requested_path.ends_with('/') {
        format!("{}index.html", full_path)
    } else {
        full_path
    };

    println!("Full Path:\n{}", full_path);

    // Get the contents of the requested file and construct the response.
    let response = match fs::read_to_string(&full_path) {
        Ok(content) => format!("HTTP/1.1 200 OK\r\n\r\n{}", content),
        Err(_) => "HTTP/1.1 404 Not Found\r\n\r\n404 Not Found".to_string(),
    };

    println!("Response:\n{}", response);

    // Send the response.
    stream.write(response.as_bytes()).unwrap(); // Converts response into bytes.
    stream.flush().unwrap(); // Immediately flushes data from buffer.
}

fn parse_http_request(request: &str) -> &str {
    // Simple parser to extract the requested path.
    let start_index = request.find(' ').unwrap_or(0) + 1;
    let end_index = request[start_index..].find(' ').unwrap_or(request.len());
    &request[start_index..start_index + end_index]
}

fn main() {
    // Bind the server to an address.
    let listener = TcpListener::bind(format!("{}:{}", ADDRESS, PORT)).unwrap();
    println!("{}", format!("Server listening on {}:{}...", ADDRESS, PORT));

    // Listen for incoming connections.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle each client concurrently.
                std::thread::spawn(|| handle_client(stream));
                // The || above creates a closure (anonymous function) that takes no args and
                // captures the stream variable from the surrounding scope.
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

