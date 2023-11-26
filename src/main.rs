use basicmathlang::{reader, tokens, parser, runner};

fn main() {
    let file_data = reader::read_file("math.bml").unwrap();
    let direct_tokens = tokens::parse_to_tokens(file_data);
    let parsered_tokens = parser::parse_tokens(&direct_tokens);

    runner::run(&parsered_tokens);
}
