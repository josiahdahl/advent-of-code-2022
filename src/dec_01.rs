use std::fs;
use std::path::Path;

#[derive(Clone)]
struct Elf {
    index: u8,
    calories: u64,
}

impl Elf {
    pub fn new(index: u8, calories: u64) -> Self {
        Elf { index, calories }
    }
}

// Find the Elf carrying the most calories
pub fn part_one() {
    let contents = get_contents();
    let mut lines = contents.lines();
    // 2. iterate over each group of line split by newlines and sum
    // 3. Store the index and sum as a max value
    let mut index = 0;
    let mut calories = 0;
    let mut foodiest_elf = Elf { index, calories };

    while let Some(line) = lines.next() {
        if line.is_empty() && calories > 0 {
            foodiest_elf = max_elf(foodiest_elf, index, calories);
            index += 1;
            calories = 0;
            continue;
        }

        calories += line.parse::<u64>().unwrap();
    }

    foodiest_elf = max_elf(foodiest_elf, index, calories);

    println!(
        "Foodiest Elf is {} with {} calories",
        foodiest_elf.index, foodiest_elf.calories
    )
}

pub fn part_two() {
    let contents = get_contents();
    let mut lines = contents.lines();
    let mut index = 0;
    let mut calories = 0;
    let mut foodiest_elfs = vec![Elf::new(index, calories); 3];

    while let Some(line) = lines.next() {
        if line.is_empty() && calories > 0 {
            foodiest_elfs = max_elfs(foodiest_elfs, Elf::new(index, calories));
            index += 1;
            calories = 0;
            continue;
        }

        calories += line.parse::<u64>().unwrap();
    }

    foodiest_elfs = max_elfs(foodiest_elfs, Elf::new(index, calories));

    let total_calories: u64 = foodiest_elfs.iter().map(|e| e.calories).sum();
    println!("Total calories: {}", total_calories)
}

fn max_elf(current_elf: Elf, index: u8, calories: u64) -> Elf {
    if current_elf.calories > calories {
        return current_elf;
    }
    return Elf { index, calories };
}

fn max_elfs(mut elves: Vec<Elf>, elf: Elf) -> Vec<Elf> {
    elves.push(elf);
    elves.sort_by(|e1, e2| e2.calories.cmp(&e1.calories));
    elves[..3].to_vec()
}

fn get_contents() -> String {
    // reading from the repo root (where Cargo is)
    let file_path = Path::new("./src/inputs/dec_01.txt");
    // 1. Read from inputs/dec_01.txt
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}
