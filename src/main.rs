extern crate monkey_lang_lib as mll;
use mll::tokens::tokens::*;
use mll::lexer::lexer::*;
use mll::repl::repl::*;
use std::io::*;

fn main() {
    let stdin: Stdin = std::io::stdin();
    let stdout: Stdout = std::io::stdout();

    
    mll::repl::repl::start(stdin, stdout);
}
