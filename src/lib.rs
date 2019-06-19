#![allow(unused_imports)]

//use std::default::Default;
use std::io::{Read, BufWriter};
use std::fs::File;
pub fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    str
}


pub mod tcp;
pub mod encoding;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
