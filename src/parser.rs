use crate::css::{Color, Declaration, Rule, Selector, SimpleSelector, Stylesheet, Unit, Value};
use crate::html::{elem, text, AttrMap, Node};
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
    pub(crate) fn parse_html(&mut self) -> Node {
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

    // Parse one simple selector, e.g.: `type#id.class1.class2.class3`
    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new(),
        };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    // universal selector
                    self.consume_char();
                }
                c if matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_') => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }
        selector
    }

    fn parse_identifier(&mut self) -> String {
        // TODO: Include U+00A0 and higher.
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_'))
    }

    // Parse a rule set: `<selectors> { <declarations> }`.
    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    // Parse a comma-separated list of selectors.
    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    self.consume_whitespace();
                }
                '{' => break, // start of declarations
                c => panic!("Unexpected character {} in selector list", c),
            }
        }
        // Return selectors with highest specificity first, for use in matching.
        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
        selectors
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::new();
        self.expect("{");
        self.consume_whitespace();
        while self.next_char() != '}' {
            declarations.push(self.parse_declaration());
            self.consume_whitespace();
        }
        self.expect("}");
        self.consume_whitespace();
        declarations
    }

    fn parse_declaration(&mut self) -> Declaration {
        let name = self.consume_while(|c| c != ':');
        self.expect(":");
        self.consume_whitespace();
        let value;
        if self.next_char() == '#' {
            value = Value::ColorValue(
                Color::try_from(self.consume_while(|c| c != ';')).expect("Failed to parse color"),
            );
        } else if matches!(self.next_char(), '0'..='9') {
            value = self.parse_length_value();
        } else {
            value = Value::Keyword(self.consume_while(|c| c != ';'))
        }
        self.expect(";");
        Declaration { name, value }
    }

    fn parse_length_value(&mut self) -> Value {
        let num = self
            .consume_while(|c| matches!(c, '0'..='9' | '.'))
            .parse::<f32>()
            .expect("Failed to parse length value");
        let unit = self.parse_length_unit();
        Value::Length(num, unit)
    }

    fn parse_length_unit(&mut self) -> Unit {
        // This is the only unit we support right now
        self.expect("px");
        Unit::Px
    }

    pub(crate) fn parse_css(&mut self) -> Stylesheet {
        let mut rules = Vec::new();
        while !self.eof() {
            rules.push(self.parse_rule())
        }
        Stylesheet { rules }
    }
}
