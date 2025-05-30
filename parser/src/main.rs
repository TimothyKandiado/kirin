use parser::parse_ast;
use scanner::scan_tokens;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        repl()
    } else {
        run_file(args.get(1).unwrap())
    }
}

fn run_file(path: &str) {
    let contents = std::fs::read_to_string(path).unwrap();

    let tokens = scan_tokens(contents.as_str()).unwrap();

    let ast = parse_ast(tokens, Some(path.to_string())).unwrap();

    ast.iter().for_each(|item| println!("{:#?}", item));
}

fn repl() {
    println!("Usage: cargo run --bin parser -- <file>");
}
