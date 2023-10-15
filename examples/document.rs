//! Parses an arbitrary document

use std::{fs::File, io::Read};

use pest::Parser;
use plaka::{Document, Rule};

fn main() {
    let mut f = File::open("./examples/data/consensus").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();

    let mut root = Document::parse(Rule::Document, &s).unwrap();
    let document = root.next().unwrap();
    assert_eq!(document.as_rule(), Rule::Document);
    assert_eq!(document.into_inner().next().unwrap().as_rule(), Rule::Item);
}
