use std::collections::{HashMap, HashSet};
use regex::Regex;
use super::utils::ParseError;

#[derive(Debug)]
enum Command {
    Mask((u64, u64)),
    Write((usize, u64)),
}

fn parse_command(s: &str) -> Result<Command, ParseError> {
    lazy_static!{
        static ref RE_MASK: Regex = Regex::new(r"^mask = ([01X]+)$").unwrap();
        static ref RE_WRITE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    }

    let is_mask = RE_MASK.is_match(s);

    if is_mask {
        let cap = RE_MASK.captures(s).ok_or(ParseError::new(&format!("Could not parse '{}' as mask", s)))?;

        let mask: String = cap[1].chars()
            .map(|v| if v == 'X' {
                '1'
            } else {
                '0'
            })
            .collect();

        let overwrite: String = cap[1].chars()
            .map(|v| if v == '1' {
                '1'
            } else {
                '0'
            })
            .collect();

        let mask = u64::from_str_radix(&mask, 2)?;
        let overwrite = u64::from_str_radix(&overwrite, 2)?;
        Ok(Command::Mask((mask, overwrite)))
    } else {
        let cap = RE_WRITE.captures(s).ok_or(ParseError::new(&format!("Could not parse '{}' as write", s)))?;
        let address = cap[1].parse::<usize>()?;
        let value = cap[2].parse::<u64>()?;
        Ok(Command::Write((address, value)))
    }
}

fn parse_input() -> Result<Vec<Command>, ParseError> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| parse_command(v))
        .collect::<Result<Vec<_>, ParseError>>()
}

pub fn problem1() -> Result<(), ParseError> {
    let commands = parse_input()?;

    let buffer_max = commands.iter()
        .map(|c| match c {
            Command::Write((a, _)) => *a,
            Command::Mask(_) => 0,
        })
        .max()
        .ok_or(ParseError::new("Could not determine memory size."))?;

    let mut memory = vec![0; buffer_max + 1];
    let mut mask = (0, 0);

    for c in &commands {
        match c {
            Command::Mask(m) => {
                mask = *m;
            },
            Command::Write((a, v)) => {
                memory[*a] = (v & mask.0) | mask.1;
            }
        }
    }

    let result: u64 = memory.iter()
        .filter(|&v| *v != 0)
        .sum();

    println!("14/1: memory init result is: {}", result);

    Ok(())
}

fn variants(mask: usize) -> Vec<usize> {
    let mut variants = HashSet::new();
    variants.insert(0);

    for i in 0..36 {
        let bit = (1_usize << i) as usize;

        let candidate = mask & bit;
        if candidate != 0 {
            variants.insert(candidate);

            let mut new_variants = vec![];
            for v in &variants {
                new_variants.push(v | candidate);
            }

            for v in new_variants {
                variants.insert(v);
            }
        }
    }

    variants.into_iter().collect()
}

fn write(memory: &mut HashMap<usize, u64>, mask: u64, overwrite: u64, address: usize, value: u64) {
    let overwrite = overwrite as usize;
    let mask = mask as usize;

    let variants = variants(mask);

    for variant in variants {
        let a = ((address | overwrite) & !mask) | variant;

        memory.entry(a).and_modify(|v| *v = value).or_insert(value);
    }
}

pub fn problem2() -> Result<(), ParseError> {
    let commands = parse_input()?;

    let mut memory = HashMap::new();
    let mut mask = (0, 0);

    for c in &commands {
        match c {
            Command::Mask(m) => {
                mask = *m;
            },
            Command::Write((a, v)) => {
                write(&mut memory, mask.0, mask.1, *a, *v);
            }
        }
    }

    let result: u64 = memory.iter()
        .map(|(_, v)| *v)
        .sum();

    println!("14/2: memory init result is: {}", result);

    Ok(())
}
