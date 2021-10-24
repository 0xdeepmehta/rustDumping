use std::str::FromStr;
use regex::Regex;
use super::utils::ParseError;

#[derive(Debug)]
struct Entry {
    from: u32,
    to: u32,
    c: char,
    password: String,
}

impl FromStr for Entry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"(\d{1,2})\-(\d{1,2})\s(\w):\s(.*)").unwrap();
        }
        let cap = RE.captures(s).unwrap();
        let from = cap[1].parse::<u32>()?;
        let to = cap[2].parse::<u32>()?;
        let c = cap[3].chars().next().unwrap();
        let password = cap[4].to_string();

        Ok(Self { from, to, c, password })
    }
}

fn parse_input() -> Result<Vec<Entry>, ParseError> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| Entry::from_str(v))
        .collect::<Result<Vec<_>, ParseError>>()
}

fn count(needle: char, haystack: &str) -> u32 {
    haystack.chars().filter(|&c| c == needle).count() as u32
}

fn is_valid_password_for_sled_rental(entry: &Entry) -> bool {
    let count = count(entry.c, &entry.password);
    count >= entry.from && count <= entry.to
}

pub fn problem1() -> Result<usize, ParseError> {
    let input = parse_input()?;

    let number_of_valid_passwords = input.iter()
        .filter(|e| is_valid_password_for_sled_rental(e))
        .count();

    println!("2/1: # of valid passwords: {}", number_of_valid_passwords);

    Ok(number_of_valid_passwords)
}

fn position_to_index(position: u32) -> usize {
    (position - 1) as usize
}

fn is_valid_password_for_toboggan_rental(entry: &Entry) -> bool {
    let chars = entry.password.chars().collect::<Vec<_>>();

    let first_char = chars[position_to_index(entry.from)];
    let second_char = chars[position_to_index(entry.to)];

    let first_position_hits = first_char == entry.c && second_char != entry.c;
    let second_position_hits = first_char != entry.c && second_char == entry.c;

    first_position_hits || second_position_hits
}

pub fn problem2() -> Result<usize, ParseError> {
    let input = parse_input()?;

    let number_of_valid_passwords = input.iter()
        .filter(|e| is_valid_password_for_toboggan_rental(e))
        .count();

    println!("2/2: # of valid passwords: {}", number_of_valid_passwords);

    Ok(number_of_valid_passwords)
}
