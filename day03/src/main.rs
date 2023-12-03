use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn find_adjacent_symbols(
    lines: &[String],
    x: usize,
    y0: usize, // inclusive
    y1: usize, // exclusive
) -> Vec<(usize, usize)> {
    let mut symbols = Vec::new();

    let x_start = x.saturating_sub(1);
    let x_end = cmp::min(x + 2, lines.len());

    for x0 in x_start..x_end {
        let y0 = y0.saturating_sub(1);
        for (y0, ch) in lines[x0].chars().enumerate().skip(y0).take(y1 + 1 - y0) {
            if ch == '*' {
                symbols.push((x0, y0));
            }
        }
    }
    return symbols;
}

fn main() {
    let file = File::open("./input.txt").unwrap();
    let reader = io::BufReader::new(file);
    let lines = reader
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>();

    let mut total = 0;
    let mut gears = HashMap::new();
    let part_regex = Regex::new(r"(\d+)").unwrap();

    for (x, line) in lines.iter().enumerate() {
        for m in part_regex.find_iter(line) {
            let part_n = m.as_str().parse::<u32>().unwrap();
            for symbol in find_adjacent_symbols(&lines, x, m.start(), m.end()) {
                gears
                    .entry(symbol)
                    .or_insert_with(|| Vec::new())
                    .push(part_n);
            }
        }
    }

    for gear_parts in gears.values() {
        if gear_parts.len() == 2 {
            total += gear_parts[0] * gear_parts[1];
        }
    }

    println!("Total = {}", total);
}
