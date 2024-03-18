use distributing_iterator::distribute_csv;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("Please provide a file path");
    let field = args.get(2).expect("Please provide a field name");
    let data = std::fs::read_to_string(path).unwrap();
    distribute_csv(&data, field, 400).unwrap();
}
