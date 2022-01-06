use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("server listening at port 6378");
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    // let mut buffer= [0; 1024];
    let mut reader = BufReader::new(&stream);
    let mut content = String::new();
    reader.read_line(&mut content).unwrap();
    // let content = String::from_utf8_lossy(&buffer[..]);
    println!("content: {}", content);
    if content.eq("PING\r\n") {
        stream.write("+PONG\r\n".as_bytes()).unwrap();
    }
    stream.flush();
}