use std::env;
use merkle_tree::root;

fn main() {
    let file_path: String = env::args().nth(1)
        .expect("First argument should be a path to the input file");

    println!("{}", root(&file_path));
}
