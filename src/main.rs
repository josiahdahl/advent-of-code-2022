use std::env;
mod dec_01;
mod dec_02;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Need to slice the String to make it static, otherwise Rust can't know if it lives for the
    // lifetime of the program
    // https://stackoverflow.com/a/23977218
    match &args[1][..] {
        "dec_01" => {
            dec_01::part_one();
            dec_01::part_two();
        }
        "dec_02" => {
            dec_02::part_one();
            dec_02::part_two();
        }
        _ => {
            println!("Please enter a valid date in the format dec_xx")
        }
    }
}
