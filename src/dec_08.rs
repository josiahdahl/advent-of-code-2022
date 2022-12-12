use std::{cmp, str::Lines};

pub fn part_one(mut lines: Lines) {
    let mut forest: Vec<Vec<usize>> = Vec::new();

    while let Some(line) = lines.next() {
        let mut row: Vec<usize> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as usize);
        }
        forest.push(row);
    }

    let width = forest.get(0).unwrap().len();
    let height = forest.len();

    let mut visible_count = 0;

    for (y, row) in forest.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            // Ignore anything in the top, bottom, or either edge
            if y == 0 || x == 0 || y == height - 1 || x == width - 1 {
                visible_count += 1;
                continue;
            }
            if visible_up(&forest, tree, x, y) {
                // println!("{tree} at ({x}, {y}) is visible from up");
                visible_count += 1;
                continue;
            }

            if visible_right(&forest, tree, x, y) {
                // println!("{tree} at ({x}, {y}) is visible from right");
                visible_count += 1;
                continue;
            }

            if visible_down(&forest, tree, x, y) {
                // println!("{tree} at ({x}, {y}) is visible from down");
                visible_count += 1;
                continue;
            }
            if visible_left(&forest, tree, x, y) {
                // println!("{tree} at ({x}, {y}) is visible from left");
                visible_count += 1;
                continue;
            }
        }
    }

    println!("Visible trees: {visible_count}");
}

fn visible_up(forest: &Vec<Vec<usize>>, tree_val: &usize, tree_x: usize, tree_y: usize) -> bool {
    // The .rev() thing is weird, this goes from tree_y - 1 down to 0
    for cur_row in (std::usize::MIN..=(tree_y - 1)).rev() {
        let up_val = &forest[cur_row][tree_x];
        // println!("Checking tree val {tree_val} on row {tree_y} against upval {up_val}");
        if up_val >= &tree_val {
            return false;
        }
    }

    // println!("visible_up: {tree_val} is visible on row {tree_y} at col {tree_x}");
    return true;
}

fn visible_right(forest: &Vec<Vec<usize>>, tree_val: &usize, tree_x: usize, tree_y: usize) -> bool {
    for cur_col in (tree_x + 1)..forest[0].len() {
        let right_val = &forest[tree_y][cur_col];
        // println!("Checking tree val {tree_val} on row {tree_y} against rightval {right_val}");
        if right_val >= &tree_val {
            return false;
        }
    }
    // println!("visible_right: {tree_val} is visible on row {tree_y} at col {tree_x}");
    return true;
}
fn visible_down(forest: &Vec<Vec<usize>>, tree_val: &usize, tree_x: usize, tree_y: usize) -> bool {
    for cur_row in (tree_y + 1)..forest.len() {
        let up_val = &forest[cur_row][tree_x];
        // println!("Checking tree val {tree_val} on row {tree_y} against downval {up_val}");
        if up_val >= &tree_val {
            return false;
        }
    }

    // println!("visible_down: {tree_val} is visible on row {tree_y} at col {tree_x}");
    return true;
}
fn visible_left(forest: &Vec<Vec<usize>>, tree_val: &usize, tree_x: usize, tree_y: usize) -> bool {
    for cur_col in (std::usize::MIN..=(tree_x - 1)).rev() {
        let left_val = &forest[tree_y][cur_col];
        // println!("Checking tree val {tree_val} on row {tree_y} against rightval {left_val}");
        if left_val >= &tree_val {
            return false;
        }
    }
    // println!("visible_left: {tree_val} is visible on row {tree_y} at col {tree_x}");
    return true;
}
pub fn part_two(mut lines: Lines) {
    let mut forest: Vec<Vec<usize>> = Vec::new();

    while let Some(line) = lines.next() {
        let mut row: Vec<usize> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as usize);
        }
        forest.push(row);
    }
    let mut max_scenic_score = std::usize::MIN;
    for (y, row) in forest.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let up = scenic_up(&forest, tree, x, y);
            let right = scenic_right(&forest, tree, x, y);
            let down = scenic_down(&forest, tree, x, y);
            let left = scenic_left(&forest, tree, x, y);
            let scenic_score = up * right * down * left;
            // println!("({x}, {y}) score: {up} {right} {down} {left} = {scenic_score}");
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
                println!("Found {scenic_score} at ({x}, {y}) with {up} {right} {down} {left}");
            }
            // max_scenic_score = cmp::max(max_scenic_score, scenic_score);
        }
    }
    println!("Part Two: Max Scenic Score {max_scenic_score}");
}
// Scenic algorithm
// 0. If you are on an edge and going in that direction, scenic is 0
// 1. Look in the direction until you find a tree that is same height or taller.
// 2. Scenic is tree_coord - bigger_coord
fn scenic_up(forest: &Vec<Vec<usize>>, tree_val: &usize, tree_x: usize, tree_y: usize) -> usize {
    if tree_y == std::usize::MIN {
        return 0;
    }
    // The .rev() thing is weird, this goes from tree_y - 1 down to 0
    for cur_row in (std::usize::MIN..=(tree_y - 1)).rev() {
        let up_val = &forest[cur_row][tree_x];
        if up_val >= &tree_val {
            return tree_y - cur_row;
        }
    }

    return tree_y;
}

fn scenic_right(forest: &Vec<Vec<usize>>, tree_val: &usize, tree_x: usize, tree_y: usize) -> usize {
    if tree_x == forest[0].len() - 1 {
        return 0;
    }
    for cur_col in (tree_x + 1)..forest[0].len() {
        let right_val = &forest[tree_y][cur_col];
        if right_val >= &tree_val {
            return cur_col - tree_x;
        }
    }
    return forest[0].len() - 1 - tree_x;
}
fn scenic_down(forest: &Vec<Vec<usize>>, tree_val: &usize, tree_x: usize, tree_y: usize) -> usize {
    if tree_y == forest.len() - 1 {
        return 0;
    }
    for cur_row in (tree_y + 1)..forest.len() {
        let down_val = &forest[cur_row][tree_x];
        if down_val >= &tree_val {
            return cur_row - tree_y;
        }
    }
    return forest.len() - 1 - tree_y;
}
fn scenic_left(forest: &Vec<Vec<usize>>, tree_val: &usize, tree_x: usize, tree_y: usize) -> usize {
    if tree_x == 0 {
        return 0;
    }
    for cur_col in (std::usize::MIN..=(tree_x - 1)).rev() {
        let left_val = &forest[tree_y][cur_col];
        if left_val >= &tree_val {
            return tree_x - cur_col;
        }
    }
    return tree_x;
}
