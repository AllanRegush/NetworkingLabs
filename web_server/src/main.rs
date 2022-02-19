use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;

fn handle_page(page_file_name: &str, mut stream: TcpStream) {
    let page = fs::read_to_string(page_file_name).unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        page.len(),
        page
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);

    let values: Vec<&str> = request.split(" ").collect();

    let file_name = String::from(&values[1][1..values[1].len()]);

    if values[1] == "/favicon.ico" {
        return;
    }

    if values[0] == "GET" && Path::new(&file_name).exists() {
        println!("Request: {}", request);
        handle_page(&file_name, stream);
    } else {
        println!("Request: {}", request);
        handle_page("404.html", stream)
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}
