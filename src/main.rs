mod compiler;
mod contexts;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod sources;
mod value;

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use ariadne::Cache;

use compiler::Compiler;
use logos::Logos;

use parser::{parse, ASTData, ParseData};
use sources::{SpwnCache, SpwnSource};

use crate::compiler::Instruction;

fn run(code: String, source: SpwnSource) {
    let code = code.trim_end().to_string();
    let mut tokens_iter = lexer::Token::lexer(&code);
    let mut tokens = vec![];
    while let Some(t) = tokens_iter.next() {
        tokens.push((t, (tokens_iter.span().start, tokens_iter.span().end)))
    }
    tokens.push((lexer::Token::Eof, (code.len(), code.len())));

    let mut cache = SpwnCache::default();
    cache.fetch(&source).expect("File does not exist!");

    let mut ast_data = ASTData::default();
    let parse_data = ParseData { source, tokens };

    let ast = parse(&parse_data, &mut ast_data);

    match ast {
        Ok(stmts) => {
            ast_data.debug(&stmts);

            let mut compiler = Compiler::new(ast_data);
            compiler.code.instructions.push(vec![]);

            compiler.compile_stmts(stmts, 0);

            compiler.code.debug();
        }
        Err(e) => {
            e.raise(cache);
        }
    }
}

fn main() {
    print!("\x1B[2J\x1B[1;1H");

    io::stdout().flush().unwrap();
    let mut buf = PathBuf::new();
    buf.push("test.spwn");
    let code = fs::read_to_string(buf.clone()).unwrap();
    run(code, SpwnSource::File(buf));

    println!("{}", std::mem::size_of::<Instruction>());
}
