extern crate monkey_lang_lib as mll;
use mll::repl::repl::{start};
use std::io::{Stdin, Stdout};

fn main() {
    let stdin: Stdin = std::io::stdin();
    let stdout: Stdout = std::io::stdout();

    start(stdin, stdout);
}
