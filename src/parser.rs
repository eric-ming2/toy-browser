use crate::node::{elem, text, AttrMap, Node};
use std::collections::HashMap;

pub struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    pub(crate) fn new(input: String) -> Self {
        Self { pos: 0, input }
    }
    // Read the current character without consuming it.
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // Do the next characters start with the given string?
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    // If the exact string `s` is found at the current position, consume it.
    // Otherwise, panic.
    fn expect(&mut self, s: &str) {
        if self.starts_with(s) {
            self.pos += s.len();
        } else {
            panic!("Expected {:?} at byte {} but it was not found", s, self.pos);
        }
    }

    // Return true if all input is consumed.
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // Return the current character, and advance self.pos to the next character.
    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        c
    }

    // Consume characters until `test` returns false.
    fn consume_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    // Consume and discard zero or more whitespace characters.
    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    // Parse a tag or attribute name.
    fn parse_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    // Parse a single node.
    fn parse_node(&mut self) -> Node {
        if self.starts_with("<") {
            if self.starts_with("<!--") {
                self.parse_comment();
                self.parse_node()
            } else {
                self.parse_element()
            }
        } else {
            self.parse_text()
        }
    }

    // Parse a comment (skip and ignore)
    fn parse_comment(&mut self) {
        self.expect("<!--");
        // TODO: Your comment can't have a dash in it or my parser will break
        let comment = self.consume_while(|c| c != '-');
        self.expect("-->");
        self.consume_whitespace();
        println!("Parsed comment: {}", comment);
    }

    // Parse a text node.
    fn parse_text(&mut self) -> Node {
        text(self.consume_while(|c| c != '<'))
    }

    // Parse a single element, including its open tag, contents, and closing tag.
    fn parse_element(&mut self) -> Node {
        // Opening tag.
        self.expect("<");
        let tag_name = self.parse_name();
        let attrs = self.parse_attributes();
        self.expect(">");

        // Contents.
        let children = self.parse_nodes();

        // Closing tag.
        self.expect("</");
        self.expect(&*tag_name.clone());
        self.expect(">");

        elem(tag_name, attrs, children)
    }

    // Parse a single name="value" pair.
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_name();
        self.expect("=");
        let value = self.parse_attr_value();
        (name, value)
    }

    // Parse a quoted value.
    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        let close_quote = self.consume_char();
        assert_eq!(open_quote, close_quote);
        value
    }

    // Parse a list of name="value" pairs, separated by whitespace.
    fn parse_attributes(&mut self) -> AttrMap {
        let mut attrs = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attrs.insert(name, value);
        }
        AttrMap { attrs }
    }

    // Parse a sequence of sibling nodes.
    fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    // Parse an HTML document and return the root element.
    pub(crate) fn parse(&mut self) -> Node {
        let mut nodes = self.parse_nodes();

        // If the document contains a root element, just return it. Otherwise, create one.
        if nodes.len() == 1 {
            nodes.remove(0)
        } else {
            elem(
                "html".to_string(),
                AttrMap {
                    attrs: HashMap::new(),
                },
                nodes,
            )
        }
    }
}
