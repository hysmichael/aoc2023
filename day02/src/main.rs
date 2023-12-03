use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Default)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    // fn contains(&self, other: &Self) -> bool {
    //     self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    // }

    fn update(&mut self, other: &Self) {
        self.red = cmp::max(self.red, other.red);
        self.green = cmp::max(self.green, other.green);
        self.blue = cmp::max(self.blue, other.blue);
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct ParseCubesError(String);

// Revised from_str method with better error handling
impl FromStr for Cubes {
    type Err = ParseCubesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut obj = Cubes::default();
        for info in s.split(",") {
            let (count, color) = info
                .trim()
                .split_once(" ")
                .ok_or(ParseCubesError("Invalid format".to_string()))?;
            let count = count
                .parse::<u32>()
                .map_err(|_| ParseCubesError("Invalid count".to_string()))?;
            match color {
                "red" => obj.red = count,
                "green" => obj.green = count,
                "blue" => obj.blue = count,
                _ => return Err(ParseCubesError("Unknown color".to_string())),
            };
        }
        Ok(obj)
    }
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (_, line) = line.split_once(":").unwrap();

        let minimal_possible = line
            .split(";")
            .map(|segment| segment.parse::<Cubes>().unwrap())
            .reduce(|mut acc, e| {
                acc.update(&e);
                acc
            })
            .unwrap();

        total += minimal_possible.power();
    }

    println!("Total = {}", total)
}
