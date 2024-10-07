use tree_sitter::{Parser, Tree};

fn main() {
    let code = include_str!("../Main.java");

    let mut parser = Parser::new();
    let language = tree_sitter_java::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Java grammar");

    let tree = parser.parse(code, None).unwrap();

    traverse(&tree);
}

fn traverse(tree: &Tree) {
    let mut cursor = tree.walk();
    let mut end = false;

    let mut indent = 0;

    loop {
        let node = cursor.node();
        if node.is_named() {
            println!("{}{node:?}", std::iter::repeat(' ').take(indent).collect::<String>());
        }

        if cursor.goto_first_child() {
            indent += 2;
        } else {
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    end = true;
                    break;
                }
                indent -= 2;
            }
        }

        if end {
            break;
        }
    }
}
