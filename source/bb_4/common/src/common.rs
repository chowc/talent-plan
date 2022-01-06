use serde::Serialize;
use serde::Deserialize;

pub static PING: usize = 1;

#[derive(Serialize, Deserialize, Debug)]
pub struct Req {
    pub cmd: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rsp {
    pub content: String,
}
