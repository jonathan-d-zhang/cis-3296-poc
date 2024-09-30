use tree_sitter::Parser;

fn main() {
    let code = r#"
    class Main {
        public static void main(String[] args) {
            System.out.println("Hello, world!");
        }
    }
    "#;

    let mut parser = Parser::new();
    let language = tree_sitter_java::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Java grammar");

    let tree = parser.parse(code, None).unwrap();

    println!("{tree:?}");
}
