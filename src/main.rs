use crate::node::text;
use crate::node::AttrMap;
use crate::node::{elem, pretty_print};
use std::collections::HashMap;

mod node;

fn main() {
    // Build this structure and pretty print it:
    // <html>
    //     <body>
    //         <h1>Title</h1>
    //         <div id="main" class="test">
    //             <p>Hello <em>world</em>!</p>
    //         </div>
    //     </body>
    // </html>

    let mut root = elem(
        "html".to_string(),
        AttrMap {
            attrs: HashMap::new(),
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

    let hello = text("Hello".to_string());
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

    pretty_print(&root, "".to_string());
}
