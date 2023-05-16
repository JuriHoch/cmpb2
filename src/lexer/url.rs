use logos::{Lexer, Logos, Source};
use std::fmt::{Display, Formatter};

/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    // TODO: Capture link definitions
     #[regex(r#"<a[^>]*href=[^"]*"[^>]*>[^<]*</a[ \t\n\f]*>"#, extract_link_info)]
    Link((LinkUrl, LinkText)),

    // TODO: Ignore all characters that do not belong to a link definition
 

    #[regex(r"<[^>]*>", logos::skip)]
    #[regex(r"[^<]*", logos::skip)]
     Ignored,

    // Catch any error
    #[error]
    Error,
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    // TODO: Implement extraction from link definition
  //  todo!()

  let slice = lex.slice();
//extract text
 let mut text = slice.split(">");
  text.next();
  let text = text.next().unwrap().split("<").next().unwrap();
//extract URL
  let mut href = slice.split("href=\"");
  href.next();
  let href = href.next().unwrap().split("\"").next().unwrap();

 


  (LinkUrl(String::from(href)), LinkText(String::from(text)))




}
