use std::{env, process};

use basicmathlang::{reader, tokens, parser, runner};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: <filename>");
        process::exit(1);
    }

    let file_data = reader::read_file(args.get(1).unwrap()).unwrap();
    let direct_tokens = tokens::parse_to_tokens(file_data);
    let parsered_tokens = parser::parse_tokens(&direct_tokens);

    runner::run(&parsered_tokens);
}
