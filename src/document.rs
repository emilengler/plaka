//! Parses arbitrary Tor network documents
//!
//! This module serves as a generic base for parsing Tor network documents,
//! as specified within the [dir-spec](https://spec.torproject.org/dir-spec/).
//!
//! You may see the use of several calls to `.unwrap()` and some assert
//! statements here, which may seem like a bad practice for production code.
//! However, you have to keep in mind that these statements are ensured to
//! always be true during runtime, as the execution has already passed the
//! grammatical analysis by the parser generator then.

use base64::Engine;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use thiserror::Error;

#[derive(Clone, Debug, Parser)]
#[grammar = "grammar/document.pest"]
/// An arbitrary Tor network document
pub struct Document {
    /// All items of this document in the order in which they occurred
    pub items: Vec<Item>,
}

#[derive(Clone, Debug)]
/// An item inside the Tor network document
pub struct Item {
    /// The mandatory keyword of the item
    pub keyword: String,
    /// The arguments provided to the item
    pub arguments: Vec<String>,
    /// The optional base64-decoded object
    pub object: Option<Vec<u8>>,
}

#[derive(Debug, Error)]
/// Errors that can occurr during the runtime
pub enum Error {
    #[error("invalid base64 in object")]
    InvalidBase64,
    #[error("the parser failed")]
    PestFailure,
    #[error("unknown parsing error occurred")]
    Unknown,
}

impl Document {
    /// Parses a string into its internal representation
    pub fn parse_str(input: &str) -> Result<Self, Error> {
        let mut token = match Self::parse(Rule::Document, input) {
            Ok(token) => token,
            Err(_) => return Err(Error::PestFailure),
        };

        Self::parse_token(token.next().unwrap())
    }

    /// Evaluates the AST of a `Document` into the internal representation
    pub fn parse_token(document: Pair<'_, Rule>) -> Result<Self, Error> {
        debug_assert_eq!(document.as_rule(), Rule::Document);

        // Evaluate each and every `Item`
        let items: Result<Vec<Item>, Error> = document
            .into_inner()
            .map(|item| Item::parse_token(item))
            .collect();
        let items = items?;

        Ok(Self { items })
    }
}

impl Item {
    /// Evaluates the AST of an `Item` into the internal representation
    pub fn parse_token(item: Pair<'_, Rule>) -> Result<Self, Error> {
        debug_assert_eq!(item.as_rule(), Rule::Item);

        let mut keyword = String::new();
        let mut arguments = Vec::new();
        let mut object = None;

        // Evaluation loop for the elements of an `Item`
        // TODO: Consider removing that with something that is linear, as we
        //       already have a guranteed structure here
        for element in item.into_inner() {
            match element.as_rule() {
                Rule::KeywordLine => (keyword, arguments) = Self::parse_keyword_line(element)?,
                Rule::Object => object = Some(Self::parse_object(element)?),
                _ => panic!("unknown element in item"), // unreachable
            }
        }

        Ok(Self {
            keyword,
            arguments,
            object,
        })
    }

    /// Evalutes the AST of a `KeywordLine` into the internal representation
    fn parse_keyword_line(keyword_line: Pair<'_, Rule>) -> Result<(String, Vec<String>), Error> {
        // Fetches the object which are guranteed to be in there
        let mut kl = keyword_line.into_inner();
        let keyword: String = kl.next().unwrap().as_str().into();
        let arguments: Vec<String> = kl.map(|argument| argument.as_str().into()).collect();

        Ok((keyword, arguments))
    }

    /// Evaluates the AST of an `Object` into the internal representation
    fn parse_object(object: Pair<'_, Rule>) -> Result<Vec<u8>, Error> {
        // Fetch the `Base64` rule that is guranteed to be there
        let b64: Vec<Pair<'_, Rule>> = object
            .into_inner()
            .filter(|element| element.as_rule() == Rule::Base64)
            .collect();
        debug_assert_eq!(b64.len(), 1);
        let b64: String = b64[0].as_str().into();

        // Decode the base64 data with the newlines removed
        match base64::engine::general_purpose::STANDARD.decode(b64.replace("\n", "")) {
            Ok(bytes) => Ok(bytes),
            Err(_) => Err(Error::InvalidBase64),
        }
    }
}
