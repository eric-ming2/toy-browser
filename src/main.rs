use crate::node::elem;
use crate::node::text;
use crate::node::AttrMap;
use crate::parser::Parser;
use std::collections::HashMap;
use std::fs;

mod node;
mod parser;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input/input1.html")?;
    let mut parser = Parser::new(input);
    let root = parser.parse();
    println!("{:?}", root);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::Node;

    fn manually_build_test_1() -> Node {
        let mut root = elem(
            "html".to_string(),
            AttrMap {
                attrs: {
                    let mut attrs = HashMap::new();
                    attrs.insert("lang".to_string(), "en".to_string());
                    attrs
                },
            },
            Vec::new(),
        );
        let body = elem(
            "body".to_string(),
            AttrMap {
                attrs: HashMap::new(),
            },
            Vec::new(),
        );
        root.children.push(body);
        let h1 = elem(
            "h1".to_string(),
            AttrMap {
                attrs: HashMap::new(),
            },
            Vec::new(),
        );
        root.children[0].children.push(h1);
        let title = text("Title".to_string());
        root.children[0].children[0].children.push(title);
        let div = elem(
            "div".to_string(),
            {
                let mut attrs = HashMap::new();
                attrs.insert("id".to_string(), "main".to_string());
                attrs.insert("class".to_string(), "test".to_string());
                AttrMap { attrs }
            },
            Vec::new(),
        );
        root.children[0].children.push(div);
        let p = elem(
            "p".to_string(),
            AttrMap {
                attrs: HashMap::new(),
            },
            Vec::new(),
        );
        root.children[0].children[1].children.push(p);
        let hello = text("Hello ".to_string());
        root.children[0].children[1].children[0]
            .children
            .push(hello);
        let em = elem(
            "em".to_string(),
            AttrMap {
                attrs: HashMap::new(),
            },
            Vec::new(),
        );
        root.children[0].children[1].children[0].children.push(em);
        let world = text("world".to_string());
        root.children[0].children[1].children[0].children[1]
            .children
            .push(world);
        let exclamation = text("!".to_string());
        root.children[0].children[1].children[0]
            .children
            .push(exclamation);
        root
    }

    fn manually_build_test_2() -> Node {
        let mut root = elem(
            "html".to_string(),
            AttrMap {
                attrs: {
                    let mut attrs = HashMap::new();
                    attrs.insert("lang".to_string(), "en".to_string());
                    attrs
                },
            },
            Vec::new(),
        );
        let head = elem(
            "head".to_string(),
            AttrMap {
                attrs: HashMap::new(),
            },
            Vec::new(),
        );
        root.children.push(head);
        let title = elem(
            "title".to_string(),
            AttrMap {
                attrs: HashMap::new(),
            },
            Vec::new(),
        );
        root.children[0].children.push(title);
        let simple_html_page = text("Simple HTML Page".to_string());
        root.children[0].children[0].children.push(simple_html_page);
        let style = elem(
            "style".to_string(),
            AttrMap {
                attrs: HashMap::new(),
            },
            Vec::new(),
        );
        root.children[0].children.push(style);
        let style_text = text(
            r#"body {
            font-family: Arial, sans-serif;
            text-align: center;
            margin: 50px;
        }
        h1 {
            color: blue;
        }
        p {
            font-size: 18px;
        }
    "#
            .to_string(),
        );
        root.children[0].children[1].children.push(style_text);
        let body = elem(
            "body".to_string(),
            AttrMap {
                attrs: HashMap::new(),
            },
            Vec::new(),
        );
        root.children.push(body);
        let h1 = elem(
            "h1".to_string(),
            AttrMap {
                attrs: HashMap::new(),
            },
            Vec::new(),
        );
        root.children[1].children.push(h1);
        let welcome_to_my_simple_page = text("Welcome to My Simple Page".to_string());
        root.children[1].children[0]
            .children
            .push(welcome_to_my_simple_page);
        let p = elem(
            "p".to_string(),
            AttrMap {
                attrs: HashMap::new(),
            },
            Vec::new(),
        );
        root.children[1].children.push(p);
        let this_is_a_basic_html_page_with_some_text =
            text("This is a basic HTML page with some text.".to_string());
        root.children[1].children[1]
            .children
            .push(this_is_a_basic_html_page_with_some_text);
        let button = elem(
            "button".to_string(),
            AttrMap {
                attrs: {
                    let mut attrs = HashMap::new();
                    attrs.insert("onclick".to_string(), "showMessage()".to_string());
                    attrs
                },
            },
            Vec::new(),
        );
        root.children[1].children.push(button);
        let button_text = text("Click Me".to_string());
        root.children[1].children[2].children.push(button_text);
        let script = elem(
            "script".to_string(),
            AttrMap {
                attrs: HashMap::new(),
            },
            Vec::new(),
        );
        root.children[1].children.push(script);
        let script_text = text(
            r#"function showMessage() {
        alert("Hello, world!");
    }
"#
            .to_string(),
        );
        root.children[1].children[3].children.push(script_text);
        root
    }

    #[test]
    fn test_parse_1() -> std::io::Result<()> {
        let input = fs::read_to_string("input/input1.html")?;
        let mut parser = Parser::new(input);
        let parsed_root = parser.parse();
        let manual_root = manually_build_test_1();
        assert_eq!(parsed_root, manual_root);
        Ok(())
    }

    #[test]
    fn test_parse_2() -> std::io::Result<()> {
        let input = fs::read_to_string("input/input2.html")?;
        let mut parser = Parser::new(input);
        let parsed_root = parser.parse();
        let manual_root = manually_build_test_2();
        assert_eq!(parsed_root, manual_root);
        Ok(())
    }
}
