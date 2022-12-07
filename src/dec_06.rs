use std::{
    collections::{HashMap, VecDeque},
    str::Lines,
};

pub fn part_one(lines: Lines) {
    let index = find_first_unique(4, lines);
    println!("First signal is at index {}", index);
    // let line = &lines.next().unwrap();
    // let signal = line.chars();
    // let mut index = 0;
    // // Track how often things have been seen
    // let mut seen: HashMap<char, u8> = HashMap::new();
    // // Track the order in which they're seen
    // let mut window: VecDeque<char> = VecDeque::new();
    // let window_size = 4;
    //
    // for s in signal {
    //     // preload the window, then start processing
    //     if index < window_size - 1 {
    //         put_seen(&s, &mut seen);
    //         window.push_back(s);
    //         index += 1;
    //         continue;
    //     }
    //
    //     index += 1;
    //     window.push_back(s);
    //     put_seen(&s, &mut seen);
    //
    //     if is_signal(window_size, &seen) {
    //         println!("First signal is at index {}", index);
    //         break;
    //     }
    //     println!("at index {index} with char {s}");
    //     let popped = window.pop_front().unwrap();
    //     pop_seen(&popped, &mut seen);
    // }
}

fn find_first_unique(unique_count: usize, mut lines: Lines) -> usize {
    let line = &lines.next().unwrap();
    let signal = line.chars();
    let mut index = 0;
    // Track how often things have been seen
    let mut seen: HashMap<char, u8> = HashMap::new();
    // Track the order in which they're seen
    let mut window: VecDeque<char> = VecDeque::new();

    for s in signal {
        // preload the window, then start processing
        if index < unique_count - 1 {
            put_seen(&s, &mut seen);
            window.push_back(s);
            index += 1;
            continue;
        }

        index += 1;
        window.push_back(s);
        put_seen(&s, &mut seen);

        if is_signal(unique_count, &seen) {
            return index;
        }
        let popped = window.pop_front().unwrap();
        pop_seen(&popped, &mut seen);
    }
    panic!("Failed to find unique!");
}

fn put_seen(s: &char, current_items: &mut HashMap<char, u8>) {
    match current_items.remove(s) {
        Some(val) => current_items.insert(*s, val + 1),
        None => current_items.insert(*s, 1),
    };
}

fn pop_seen(popped: &char, current_items: &mut HashMap<char, u8>) {
    match current_items.remove(popped) {
        Some(val) => {
            if val != 1 {
                current_items.insert(*popped, val - 1);
            }
        }
        None => panic!("We shouldn't be popping values that don't exist..."),
    }
}

fn is_signal(signal_size: usize, current_items: &HashMap<char, u8>) -> bool {
    return current_items.len() == signal_size;
}

pub fn part_two(lines: Lines) {
    let index = find_first_unique(14, lines);
    println!("First message is at index {}", index);
}
