use std::str::Lines;

struct Section {
    start: u32,
    end: u32,
}

impl Section {
    pub fn new(start: u32, end: u32) -> Self {
        Section { start, end }
    }

    pub fn contains(self: &Self, section: &Section) -> bool {
        return self.start <= section.start && self.end >= section.end;
    }
}

pub fn part_one(mut lines: Lines) {
    let mut overlapping_sections = 0;
    while let Some(line) = lines.next() {
        let mut string_sections = line.split(",");
        let section_one = section_from_string(string_sections.next().unwrap());
        let section_two = section_from_string(string_sections.next().unwrap());

        if section_one.contains(&section_two) || section_two.contains(&section_one) {
            overlapping_sections += 1;
        }
    }

    println!("Part One: overlapping sections {overlapping_sections}");
}

fn section_from_string(section: &str) -> Section {
    let mut start_end = section.split("-");
    let start: u32 = start_end.next().unwrap().parse().unwrap();
    let end: u32 = start_end.next().unwrap().parse().unwrap();
    return Section::new(start, end);
}
pub fn part_two(_lines: Lines) {}
