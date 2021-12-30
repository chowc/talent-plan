use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use log::info;

static StringPrefix: &str = "+";
static ErrorPrefix: &str = "-";
static IntPrefix: &str = ":";
static BulkStringPrefix: &str = "$";
static ArrayPrefix: &str = "*";

fn main() {
    // 定义一个请求地址 IP:端口 的形式
    let addr = "127.0.0.1:6379";
    let mut stream = TcpStream::connect(addr).unwrap();
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    writer.write("PING\r\n".as_bytes()).unwrap();
    writer.flush();
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();

    println!("read {}", s);
    writer.write("GET a\r\n".as_bytes()).unwrap();
    writer.flush();
    let mut s = String::new();
    reader.read_line(&mut s).unwrap();
    println!("read {}", s);
}
