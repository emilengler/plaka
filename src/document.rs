use base64::Engine;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use thiserror::Error;

#[derive(Clone, Debug, Parser)]
#[grammar = "grammar/document.pest"]
pub struct Document {
    pub items: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub keyword: String,
    pub arguments: Vec<String>,
    pub object: Option<Vec<u8>>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid base64 in object")]
    InvalidBase64,
    #[error("the parser failed")]
    PestFailure,
    #[error("unknown parsing error occurred")]
    Unknown,
}

impl Document {
    pub fn parse_str(input: &str) -> Result<Self, Error> {
        let mut token = match Self::parse(Rule::Document, input) {
            Ok(token) => token,
            Err(_) => return Err(Error::PestFailure),
        };

        Self::parse_token(token.next().unwrap())
    }

    pub fn parse_token(document: Pair<'_, Rule>) -> Result<Self, Error> {
        debug_assert_eq!(document.as_rule(), Rule::Document);

        let items: Result<Vec<Item>, Error> = document
            .into_inner()
            .map(|item| Item::parse_token(item))
            .collect();
        let items = items?;

        Ok(Self { items })
    }
}

impl Item {
    pub fn parse_token(item: Pair<'_, Rule>) -> Result<Self, Error> {
        debug_assert_eq!(item.as_rule(), Rule::Item);

        let mut keyword = String::new();
        let mut arguments = Vec::new();
        let mut object = None;

        for element in item.into_inner() {
            match element.as_rule() {
                Rule::KeywordLine => (keyword, arguments) = Self::parse_keyword_line(element)?,
                Rule::Object => object = Some(Self::parse_object(element)?),
                _ => panic!("unknown element in item"),
            }
        }

        Ok(Self {
            keyword,
            arguments,
            object,
        })
    }

    fn parse_keyword_line(keyword_line: Pair<'_, Rule>) -> Result<(String, Vec<String>), Error> {
        let mut kl = keyword_line.into_inner();
        let keyword: String = kl.next().unwrap().as_str().into();
        let arguments: Vec<String> = kl.map(|argument| argument.as_str().into()).collect();

        Ok((keyword, arguments))
    }

    fn parse_object(object: Pair<'_, Rule>) -> Result<Vec<u8>, Error> {
        let b64: Vec<Pair<'_, Rule>> = object
            .into_inner()
            .filter(|element| element.as_rule() == Rule::Base64)
            .collect();
        debug_assert_eq!(b64.len(), 1);
        let b64: String = b64[0].as_str().into();

        match base64::engine::general_purpose::STANDARD.decode(b64.replace("\n", "")) {
            Ok(bytes) => Ok(bytes),
            Err(_) => Err(Error::InvalidBase64),
        }
    }
}