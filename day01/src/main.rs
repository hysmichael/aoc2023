use std::fs::File;
use std::io::{self, BufRead};

const STR_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let mut digits = Vec::<u32>::new();
        for (idx, ch) in line.char_indices() {
            if let Some(digit) = ch.to_digit(10) {
                digits.push(digit);
            } else {
                if let Some((k, _)) = STR_DIGITS
                    .iter()
                    .enumerate()
                    .find(|(_, &word)| (&line[idx..]).starts_with(word))
                {
                    digits.push(k as u32 + 1); // offset by 1
                }
            }
        }

        assert!(!digits.is_empty());
        total += digits.first().unwrap() * 10 + digits.last().unwrap();
    }

    println!("Total = {}", total)
}
