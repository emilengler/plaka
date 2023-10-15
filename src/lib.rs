use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/document.pest"]
pub struct Document;
