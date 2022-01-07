#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod common;
pub use common::Req;
pub use common::Rsp;
pub use common::PING;