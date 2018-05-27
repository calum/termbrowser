use actix_web::ResponseError;
use md::push_cli;
use pulldown_cmark::Parser;
use reqwest;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TlDrError {
    Http(reqwest::Error),
}

impl From<reqwest::Error> for TlDrError {
    fn from(error: reqwest::Error) -> Self {
        TlDrError::Http(error)
    }
}

impl Error for TlDrError {
    fn description(&self) -> &str {
        match self {
            TlDrError::Http(error) => error.description(),
        }
    }
}

impl fmt::Display for TlDrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            TlDrError::Http(error) => write!(f, "{}", error),
        }
    }
}

impl ResponseError for TlDrError {}

fn get_man(name: &str) -> Result<String, TlDrError> {
    Ok(reqwest::get(
        &("https://raw.githubusercontent.com/tldr-pages/tldr/master/pages/common/".to_string()
            + name + ".md"),
    )?.text()?)
}

pub fn render_man(name: &str) -> Result<String, TlDrError> {
    let md = get_man(name)?;
    let parser = Parser::new(&md);
    let mut buf = String::new();
    push_cli(&mut buf, parser);
    Ok(buf)
}
