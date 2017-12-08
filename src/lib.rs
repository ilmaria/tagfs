extern crate futures;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

mod ipfs;

#[cfg(test)]
mod tests {
    use super::ipfs;

    #[test]
    fn it_works() {
        let s = "hei";
        let a = ipfs::add(&s);
        assert_eq!(a + 1, 4);
    }
}
