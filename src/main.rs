use std::fs;

mod lexer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    run_driver(args);
}

fn run_driver(args: Vec<String>) {
    let source_c_file_name = args
        .iter()
        .skip(1)
        .find(|arg| !arg.starts_with("--"))
        .expect("No input source file provided !")
        .to_owned();

    // --lex runs the lexer, but stops before parsing
    let option_lex = args.iter().any(|arg| arg == "--lex");
    // --parse runs the lexer and parser, but stops before assembly generation
    let option_parse = args.iter().any(|arg| arg == "--parse");
    // --codegen runs the lexer, parser, and assembly generation but stops before code emission
    let option_codegen = args.iter().any(|arg| arg == "--codegen");

    // 1 - preprocess the source file
    let preprocessed_name = source_c_file_name
        .strip_suffix(".c")
        .map(|s| format!("{}.i", s))
        .expect("Input file is not a .c file !");

    let preprocess = std::process::Command::new("gcc")
        .args(["-E", "-P", &source_c_file_name, "-o", &preprocessed_name])
        .output()
        .expect("Failed to execute gcc for preprocessing.");

    if !preprocess.status.success() {
        panic!(
            "Error during preprocessing : {}",
            String::from_utf8_lossy(&preprocess.stdout)
        );
    }

    let preprocessed_source = fs::read_to_string(&preprocessed_name).unwrap();
    fs::remove_file(&preprocessed_name).unwrap();

    // 2 - output an assembly file with a .s extension

    let tokens = lexer::lex(preprocessed_source);
    dbg!(tokens);
}
