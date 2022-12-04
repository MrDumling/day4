use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Debug)]
struct CleaningRange {
    left: u64,
    right: u64,
}

impl CleaningRange {
    fn encompasses_range(&self, compared_range: &Self) -> bool {
        self.left <= compared_range.left && self.right >= compared_range.right
    }

    fn overlaps_range(&self, compared_range: &Self) -> bool {
        self.left <= compared_range.right && compared_range.left <= self.right
    }
}

fn get_input(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
}

fn get_pair(input_line: String) -> (CleaningRange, CleaningRange) {
    let Some((range_a, range_b)) = input_line.split_once(",") else {
        panic!("Expected ',' to split ranges");
    };

    (get_range(range_a), get_range(range_b))
}

fn get_range(input: &str) -> CleaningRange {
    let Some((left, right)) = input.split_once("-") else {
        panic!("Unable to parse range that does not contain '-'");
    };
    CleaningRange {
        left: left.parse().unwrap(),
        right: right.parse().unwrap(),
    }
}

fn get_pairs(lines: Lines<BufReader<File>>) -> Vec<(CleaningRange, CleaningRange)> {
    lines.into_iter().map(|x| get_pair(x.unwrap())).collect()
}

fn puzzle_1() {
    let lines = get_input("./input.txt");

    let pairs = get_pairs(lines);

    let count = pairs
        .into_iter()
        .filter(|(x, y)| x.encompasses_range(y) || y.encompasses_range(x))
        .count();

    println!("{:#?}", count);
}

fn puzzle_2() {
    let lines = get_input("./input.txt");

    let pairs = get_pairs(lines);

    let count = pairs
        .into_iter()
        .filter(|(x, y)| x.overlaps_range(y))
        .count();

    println!("{:#?}", count);
}

fn main() {
    puzzle_1();
    puzzle_2();
}
