
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use common::{PING, Req, Rsp};

fn main() {
    println!("server listening at port 6378");
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer= [0; 1024];
    let size = stream.read(&mut buffer).unwrap();
    if size == 0 {
        panic!("read size = 0")
    }
    //
    let req: Req = serde_json::from_slice(&buffer[0..size]).expect("json from_slice fail");
    println!("got request: {:?}", req);
    if req.cmd == PING {
        let rsp = Rsp{content: String::from("PONG")};
        let bs = serde_json::to_vec(&rsp).expect("serde to_string fail");
        stream.write(bs.as_slice()).unwrap();
    }
    stream.flush();
}

