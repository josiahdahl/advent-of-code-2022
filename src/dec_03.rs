use std::panic;
use std::process::exit;
use std::{collections::HashSet, str::Lines};

pub fn part_one(mut lines: Lines) {
    // For each rucksack
    // Split into two
    // Find the character that exist in the first half and the second half
    // Sum the priorities
    let mut priorities: u32 = 0;
    while let Some(line) = lines.next() {
        let count = line.chars().count();
        assert!(count % 2 == 0);
        let mut items = HashSet::new();
        let mut chars = line.chars();
        for _ in 0..(count / 2) {
            let item = chars.next().unwrap();
            items.insert(item);
        }
        for _ in 0..(count / 2) {
            let item = chars.next().unwrap();
            if items.contains(&item) {
                let priority = panic::catch_unwind(|| item_priority(item));
                if priority.is_err() {
                    println!("Failed to get priority of char {item} in string {line}");
                    exit(1);
                }
                let val = priority.unwrap();
                priorities += val;
                break;
            }
        }
    }

    println!("Part One: Total priority is {priorities}")
}

fn item_priority(item: char) -> u32 {
    let code_point = item as u32;
    // A - Z
    if code_point >= 65 && code_point <= 90 {
        // A's code point is 65, Z's is 90
        return code_point - 65 + 27;
    } else {
        // a's code point is 97, etc
        return code_point - 97 + 1;
    }
}
pub fn part_two(mut lines: Lines) {
    let mut priorities: u32 = 0;
    let mut line_count = 0;
    let mut group_one: HashSet<char> = HashSet::new();
    let mut group_two: HashSet<char> = HashSet::new();
    let mut group_three: HashSet<char> = HashSet::new();
    while let Some(line) = lines.next() {
        line_count += 1;
        if line_count > 3 {
            priorities += get_auth_sticker_priority(&group_one, &group_two, &group_three);
            group_one.clear();
            group_two.clear();
            group_three.clear();
            line_count = 1;
        }
        let mut chars = line.chars();
        while let Some(letter) = chars.next() {
            match line_count {
                1 => group_one.insert(letter),
                2 => group_two.insert(letter),
                3 => group_three.insert(letter),
                _ => panic!("Wrong neighborhood, friend."),
            };
        }
    }

    // Make sure we add the last one in, this was a tricky bug
    priorities += get_auth_sticker_priority(&group_one, &group_two, &group_three);
    println!("Part Two: Total priority is {priorities}")
}

fn get_auth_sticker_priority(
    group_one: &HashSet<char>,
    group_two: &HashSet<char>,
    group_three: &HashSet<char>,
) -> u32 {
    for unique in group_one.iter() {
        println!(
            "matching on {unique} group_two? {} group three? {}",
            group_two.contains(unique),
            group_three.contains(unique),
        );
        if group_two.contains(unique) && group_three.contains(unique) {
            println!("Found {unique}");
            return item_priority(*unique);
        }
    }
    return 0;
}
