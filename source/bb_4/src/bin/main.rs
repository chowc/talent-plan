use std::io::{Read, Write};
use std::net::TcpStream;
use common::{PING, Req, Rsp};

fn main() {
    // 定义一个请求地址 IP:端口 的形式
    let addr = "127.0.0.1:6379";
    let mut stream = TcpStream::connect(addr).unwrap();
    let req = Req { cmd: PING };
    let buf = serde_json::to_vec(&req).expect("to_vec fail");
    stream.write(&buf[..]).expect("write fail");
    stream.flush();
    let mut rsp_buf = [0; 1024];
    let size = stream.read(&mut rsp_buf).expect("read from stream fail");
    if size == 0 {
        panic!("read from remote server size = 0");
    }
    let rsp: Rsp = serde_json::from_slice(&rsp_buf[0..size]).expect("from_slice fail");
    println!("response {:?}", rsp);
}