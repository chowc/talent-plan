use std::{fs, string};
use std::fs::File;
use std::io::{Read, Write};
use bson::{Bson, doc, Document};
use byteorder::{ByteOrder, LittleEndian};
use serde::{Deserialize, Serialize};
use serde::ser::Error;
use serde_json::{from_str, Result, to_string_pretty};
use ron::ser;
use ron::de;


fn main() -> Result<()> {

    // serde_to_json(&a);
    // serde_to_ron(&a);
    serde_to_v8_json();
    Ok(())
}

fn serde_to_json(a: &Move) {
    println!("a is {:?}", a);
    let js = serde_json::to_string(&a).expect("serde to_string fail");
    println!("js: {}", js);
    let u8v: Vec<u8> = serde_json::to_vec(&a).expect("serde to Vec<u8> fail");
    println!("Vec<u8>: {}", String::from_utf8(u8v.to_vec()).expect("Vec<u8> to utf8 fail"));
    fs::write("serde.json", js).expect("write to file fail");

    let content = fs::read_to_string("serde.json").expect("read from file fail");
    let b: Move = serde_json::from_str(&content).expect("serde from_string fail");
    println!("b is {:?}", b);
}

fn serde_to_v8_json() {
    let mut f = File::create("serde.json.v8").expect("open file fail");

    for i in 0..1000 {
        let a = Move {
            direction: Direction::Up,
            squares: i,
            s: String::from("中国"),
        };
        let js = serde_json::to_vec(&a).expect("serde to_string fail");
        let mut size_buffer: Vec<u8> = Vec::with_capacity(4);
        size_buffer.write(&js.len().to_le_bytes());
        f.write(&size_buffer).expect("write js len fail");
        f.write(&js).expect("write js to file fail");
    }
    f.flush();
    let mut rf = File::open("serde.json.v8").expect("open file fail");
    for i in 0..10000 {
        let mut size_buffer = vec![0u8; 8];
        rf.read_exact(&mut size_buffer).expect("read size buffer from file fail");

        let mut bs = [0; 2];
        LittleEndian::read_u32_into(&*size_buffer, &mut bs);
        println!("size_buffer: {}, obj_size: {}", String::from_utf8(size_buffer).expect("from_utf8 fail"), bs[0]);
        if bs[0] == 0 {
            println!("reach end, bye.");
            break;
        }
        let mut content_buffer = vec![0u8; bs[0] as usize];
        rf.read_exact(&mut content_buffer).expect("read content buffer from file fail");
        let s = match std::str::from_utf8(&*content_buffer) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let new_move: Move = serde_json::from_str(s).expect("json from_slice fail");
        println!("new_move i {}: {:?}", i, new_move);
    }

    // let b: Move = serde_json::from_str(String::from_utf8(content_buffer.to_vec())).expect("serde from_string fail");
}

fn serde_to_bon(a: &Move) {
    println!("a is {:?}", a);
    let bs = bson::to_bson(&a).expect("serde to_string fail");
    println!("bs: {}", bs);
    let mut f = File::create("serde.bon").expect("open file fail");
    for _ in 0..1000 {
        let buffer = bson::to_vec(&bs).expect("bson to vec<u8> fail");
        f.write(&buffer).expect("write to file fail");
    }
    f.flush().expect("flush fail");
    let mut rf = File::open("serde.bon").expect("open file fail");
    // let content = fs::read("serde.bon").expect("read from file fail");
    while let Ok(deserialized) = Document::from_reader(&mut rf) {
        println!("deserialized: {:?}", deserialized);
    }
}

fn serde_to_ron(a: &Move) {
    println!("a is {:?}", a);
    let js = ron::ser::to_string(&a).expect("serde to_string fail");
    println!("js: {}", js);

    fs::write("serde.ron", js).expect("write to file fail");

    let content = fs::read_to_string("serde.ron").expect("read from file fail");
    let b: Move = ron::de::from_str(&content).expect("serde from_string fail");
    println!("b is {:?}", b);
}

#[derive(Serialize, Deserialize, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    direction: Direction,
    squares: i32,
    s: String,
}