use std::env;
mod dec_01;
mod dec_02;
mod dec_03;
mod dec_04;
mod dec_05;
mod dec_06;
mod dec_07;
mod dec_08;
mod dec_10;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Need to slice the String to make it static, otherwise Rust can't know if it lives for the
    // lifetime of the program
    // https://stackoverflow.com/a/23977218
    let date = &args[1][..];
    let contents = utils::get_input(date);
    match date {
        "dec_01" => {
            dec_01::part_one();
            dec_01::part_two();
        }
        "dec_02" => {
            dec_02::part_one();
            dec_02::part_two();
        }
        "dec_03" => {
            dec_03::part_one(contents.lines());
            dec_03::part_two(contents.lines());
        }
        "dec_04" => {
            dec_04::part_one(contents.lines());
            dec_04::part_two(contents.lines());
        }
        "dec_05" => {
            dec_05::part_one(contents.lines());
            dec_05::part_two(contents.lines());
        }
        "dec_06" => {
            dec_06::part_one(contents.lines());
            dec_06::part_two(contents.lines());
        }
        "dec_07" => {
            dec_07::part_one(contents.lines());
            dec_07::part_two(contents.lines());
        }
        "dec_08" => {
            dec_08::part_one(contents.lines());
            dec_08::part_two(contents.lines());
        }
        "dec_10" => {
            dec_10::part_one(contents.lines());
            dec_10::part_two(contents.lines());
        }
        _ => {
            println!("Please enter a valid date in the format dec_xx")
        }
    }
}
