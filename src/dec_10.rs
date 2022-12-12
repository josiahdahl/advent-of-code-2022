use std::str::Lines;

pub fn part_one(mut lines: Lines) {
    let mut tick = 0;
    let mut register: i32 = 1;
    while let Some(line) = lines.next() {
        let action = line.split(" ").collect::<Vec<&str>>();
        match action[..] {
            ["noop"] => {
                tick += 1;
                maybe_print_val(tick, register);
            }
            ["addx", val] => {
                tick += 1;
                maybe_print_val(tick, register);
                tick += 1;
                // print value before incrementing because the current tick is the one we're "in"
                // right now, value increments after
                maybe_print_val(tick, register);
                register += val.to_string().parse::<i32>().unwrap();
            }
            _ => panic!("Shouldn't find a line with this value.."),
        }
    }

    println!("Tick {tick} - value: {register}");
}

fn maybe_print_val(tick: usize, register: i32) {
    if tick == 20 || tick == 60 || tick == 100 || tick == 140 || tick == 180 || tick == 220 {
        println!(
            "Tick {tick} - regiser {}, signal strength: {}",
            register,
            register * tick as i32
        );
    }
}

pub fn part_two(mut lines: Lines) {
    let mut tick = 0;
    let mut register: i32 = 1;
    while let Some(line) = lines.next() {
        let action = line.split(" ").collect::<Vec<&str>>();
        match action[..] {
            ["noop"] => {
                draw(tick, register);
                tick += 1;
            }
            ["addx", val] => {
                draw(tick, register);
                tick += 1;
                draw(tick, register);
                tick += 1;
                // print value before incrementing because the current tick is the one we're "in"
                // right now, value increments after
                register += val.to_string().parse::<i32>().unwrap();
            }
            _ => panic!("Shouldn't find a line with this value.."),
        }
    }
}

fn draw(tick: usize, register: i32) {
    let line_pos = tick % 40;
    let lower_bound = register - 1;
    let upper_bound = register + 1;
    // We could probably make this more performant by having the strings as constants
    let mut to_print = String::from(".");
    if line_pos as i32 >= lower_bound && line_pos as i32 <= upper_bound {
        to_print = String::from("#");
    }
    if line_pos == 39 {
        println!("{to_print}");
    } else {
        print!("{to_print}");
    }
}
