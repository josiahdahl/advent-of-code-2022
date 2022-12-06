use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::str::Lines;

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Move {
    pub fn new(amount: usize, from: usize, to: usize) -> Self {
        Move { amount, from, to }
    }
    pub fn parse(instructions: &str) -> Self {
        lazy_static! {
            static ref MOVE_PARSER: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }
        let captures = MOVE_PARSER.captures(instructions).unwrap();
        let amount = captures
            .get(1)
            .map(|a| a.as_str().parse::<usize>().unwrap())
            .unwrap();
        let from = captures
            .get(2)
            .map(|a| a.as_str().parse::<usize>().unwrap() - 1)
            .unwrap();
        let to = captures
            .get(3)
            .map(|a| a.as_str().parse::<usize>().unwrap() - 1)
            .unwrap();
        Move::new(amount, from, to)
    }
}
pub fn part_one(lines: Lines) {
    let mut state = get_initial_state(lines.to_owned());
    let moves = get_moves(lines.to_owned());

    for cur in moves {
        let from = pop_from(state.get_mut(cur.from).unwrap(), cur.amount);
        push_to(state.get_mut(cur.to).unwrap(), from);
    }

    for stack in state {
        print!("{}", stack.get(0).unwrap_or(&'\0'));
    }
}

pub fn part_two(lines: Lines) {
    let mut state = get_initial_state(lines.to_owned());
    let moves = get_moves(lines.to_owned());

    for cur in moves {
        let from = pop_from(state.get_mut(cur.from).unwrap(), cur.amount);
        push_many_to(state.get_mut(cur.to).unwrap(), from);
    }

    println!("");
    println!("Part Two");
    for stack in state {
        print!("{}", stack.get(0).unwrap_or(&'\0'));
    }
}

fn pop_from(stack: &mut VecDeque<char>, amount: usize) -> Vec<char> {
    let mut result: Vec<char> = vec![];
    for _ in 0..amount {
        result.push(stack.pop_front().unwrap());
    }
    result
}

// Used for part two, LIFO
fn push_many_to(stack: &mut VecDeque<char>, items: Vec<char>) {
    for item in items.into_iter().rev() {
        stack.push_front(item);
    }
}

// used for part one, FIFO
fn push_to(stack: &mut VecDeque<char>, items: Vec<char>) {
    for item in items {
        stack.push_front(item);
    }
}
fn get_initial_state(mut lines: Lines) -> Vec<VecDeque<char>> {
    let mut state: Vec<VecDeque<char>> = vec![];

    while let Some(line) = lines.next() {
        // This is the stack count line
        if line.starts_with(" 1") {
            continue;
        }
        if line == "" {
            break;
        }
        let chars: Vec<char> = line.chars().collect();
        let mut stack_index = 0;
        let chunks = chars.chunks(4);
        for chunk in chunks {
            let mut current_stack: VecDeque<char> = VecDeque::new();
            match state.get(stack_index) {
                Some(stack) => current_stack = stack.to_owned(),
                None => {}
            };
            match chunk.get(1).unwrap() {
                &' ' => {}
                val => {
                    current_stack.push_back(val.to_owned());
                }
            };
            if state.len() < stack_index + 1 {
                state.push(current_stack);
            } else {
                state[stack_index] = current_stack;
            }
            stack_index += 1;
        }
    }

    return state;
}

fn get_moves(lines: Lines) -> Vec<Move> {
    return lines
        .skip_while(|line| line != &"")
        .skip(1)
        .map(Move::parse)
        .collect();
}
