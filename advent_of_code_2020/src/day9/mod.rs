use itertools::Itertools;
use super::utils::ParseError;

fn parse_input() -> Vec<u64> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| v.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
}

fn is_valid(number: u64, preamble: &[u64]) -> bool {
    preamble.iter()
        .combinations(2)
        .any(|v| v[0] + v[1] == number)
}

fn find_invalid_number(input: &Vec<u64>, preamble_length: usize) -> (u64, usize) {
    let result = input.iter()
        .enumerate()
        .skip(preamble_length)
        .map(|(i, v)| (v, i, is_valid(*v, &input[i-preamble_length..i])))
        .filter(|(_, _, valid)| !*valid)
        .collect::<Vec<_>>();

    if result.len() != 1 {
        println!("Unexpected number of results: {}", result.len());
    }

    (*result[0].0, result[0].1)
}

pub fn problem1() -> Result<(u64, usize), ParseError> {
    let input = parse_input();

    let result = find_invalid_number(&input, 25);
    println!("9/1: invalid number: {}", result.0);

    Ok(result)
}

fn checksum(v: &[u64]) -> u64 {
    let max = v.iter().max().unwrap();
    let min = v.iter().min().unwrap();

    *max + *min
}

fn check_window_size(relevant_numbers: &[u64], window_size: usize, invalid_number: u64) -> Option<u64> {
    // TODO: early exit?
    let sum: Vec<(u64, u64)> = relevant_numbers
        .windows(window_size)
        .map(|v: &[u64]| (v.iter().sum(), checksum(v)))
        .collect::<Vec<_>>();

    let checksum = sum.iter()
        .filter(|(sum, _)| *sum == invalid_number)
        .collect::<Vec<_>>();

    if checksum.len() == 1 {
        Some(checksum[0].1)
    } else {
        None
    }
}

pub fn problem2() -> Result<(), ParseError> {
    let input = parse_input();
    let (invalid_number, position) = find_invalid_number(&input, 25);

    let relevant_numbers = &input[0..position];

    let result = (2..relevant_numbers.len()/2)
        .find_map(|window| check_window_size(relevant_numbers, window, invalid_number));

    if let Some(checksum) = result {
        println!("9/2: encryption weakness: {}", checksum);
    } else {
        println!("Found nothing :(");
    }

    Ok(())
}