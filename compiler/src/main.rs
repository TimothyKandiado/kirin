use compiler::Compiler;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: cargo run --bin compiler -- <file.kn>");
        return;
    }

    compile_file(args[1].as_str());
}

fn compile_file(path: &str) {
    let source = std::fs::read_to_string(path).unwrap();
    let tokens = scanner::scan_tokens(source.as_str()).unwrap();
    let ast = parser::parse_ast(tokens, Some(path.to_string())).unwrap();
    let analyzed_ast = analyzer::TypeChecker::new().infer_types(&ast).unwrap();

    let mut compiler = Compiler::new();
    let result = compiler.compile(&analyzed_ast);

    if result.is_err() {
        println!("Compilation failed: {:?}", result.err().unwrap());
        return;
    }

    let program = compiler.emit_program();

    println!("Program: {:?}", program);
}
