use std::fs;

use assembly::generate_assembly;
use emission::emit_code;
use parser::parse;

mod assembly;
mod emission;
mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    run_driver(args);
}

fn run_driver(args: Vec<String>) {
    let c_source_file_name = args
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
    let preprocessed_file_name = c_source_file_name
        .strip_suffix(".c")
        .map(|s| format!("{}.i", s))
        .expect("Input file is not a .c file !");

    let preprocess = std::process::Command::new("gcc")
        .args([
            "-E",
            "-P",
            &c_source_file_name,
            "-o",
            &preprocessed_file_name,
        ])
        .output()
        .expect("Failed to execute gcc for preprocessing.");

    if !preprocess.status.success() {
        panic!(
            "Error during preprocessing : {}",
            String::from_utf8_lossy(&preprocess.stdout)
        );
    }

    let preprocessed_source_string = fs::read_to_string(&preprocessed_file_name).unwrap();
    fs::remove_file(&preprocessed_file_name).unwrap();

    // 2 - output an assembly file with a .s extension

    // 2.1 - Lexing

    let tokens: Vec<lexer::Token> = lexer::lex(preprocessed_source_string);
    println!("Tokens :\n{tokens:?}");
    if option_lex {
        return;
    }

    // 2.2 - Parsing

    let ast = parse(tokens);
    println!("AST :\n{ast:#?}");
    if option_parse {
        return;
    }

    // 2.3 - Assembly Generation

    let assembly_ast = generate_assembly(ast);
    println!("Assembly AST :\n{assembly_ast:#?}");
    if option_codegen {
        return;
    }

    // 2.4 - Code Emission
    let code = emit_code(assembly_ast);
    let code_emission_file_name = c_source_file_name
        .strip_suffix(".c")
        .map(|s| format!("{}.s", s))
        .expect("Input file is not a .c file !");

    println!("Assembly:\n{code}");

    std::fs::write(&code_emission_file_name, code).unwrap();

    // 3 - Assemble and link the assembly file
    let assembled_file_name = c_source_file_name
        .strip_suffix(".c")
        .expect("Input file is not a .c file !");

    let assemble = std::process::Command::new("gcc")
        .args([&code_emission_file_name, "-o", &assembled_file_name])
        .output()
        .expect("Failed to execute gcc for assembly and linking.");

    fs::remove_file(&code_emission_file_name).unwrap();

    if !assemble.status.success() {
        panic!(
            "Error during assembly : {}",
            String::from_utf8_lossy(&assemble.stdout)
        );
    }
}
