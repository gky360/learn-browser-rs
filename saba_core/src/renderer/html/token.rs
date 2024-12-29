use crate::renderer::html::attribute::Attribute;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlTokenizer {
    state: State,
    pos: usize,
    re_consume: bool,
    latest_token: Option<HtmlToken>,
    input: Vec<char>,
    buf: String,
}

impl HtmlTokenizer {
    pub fn new(html: String) -> Self {
        Self {
            state: State::Data,
            pos: 0,
            re_consume: false,
            latest_token: None,
            input: html.chars().collect(),
            buf: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HtmlToken {
    StartTag {
        tag: String,
        self_closing: bool,
        attributes: Vec<Attribute>,
    },
    EndTag {
        tag: String,
    },
    Char(char),
    Eof,
}
