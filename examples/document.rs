//! Parses an arbitrary document

use std::{fs::File, io::Read};

use plaka::document::Document;

fn main() {
    let mut f = File::open("./examples/data/consensus").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let doc = Document::parse_str(&s).unwrap();
    println!("{:#?}", doc);
}
