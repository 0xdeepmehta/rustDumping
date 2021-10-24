use regex::Regex;
use std::str::FromStr;
use super::utils::ParseError;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^(\w+)\s([\+-]\d+)$").unwrap();
        }

        let cap = RE.captures(s).ok_or(ParseError::new(&format!("Could not parse instruction '{}'.", s)))?;

        // todo: resolve double Result
        let param = cap.get(2).map(|v| v.as_str().parse::<i32>()).ok_or(ParseError::new(&format!("Could not parse parameter of instruction '{}'.", s)))??;
        match cap.get(1).map(|m| m.as_str()).ok_or(ParseError::new(&format!("Could not parse instruction operator '{}'.", s)))? {
            "acc" => Ok(Instruction::Acc(param)),
            "jmp" => Ok(Instruction::Jmp(param)),
            "nop" => Ok(Instruction::Nop(param)),
            _ => Err(ParseError::new(&format!("Invalid instruction '{}'", s)))
        }
    }
}

fn parse_input() -> Result<Vec<Instruction>, ParseError> {
    let input = include_str!("./data/input.txt");
    let instructions = input
        .lines()
        .filter(|v| *v != "")
        .map(|v| Instruction::from_str(v))
        .collect::<Result<Vec<_>, ParseError>>()?;

    Ok(instructions)
}

fn run(instructions: Vec<Instruction>) -> (i32, bool) {
    let mut ip: i32 = 0;
    let mut acc: i32 = 0;
    let mut instructions_with_visitation = instructions.into_iter()
        .map(|i| (i, false))
        .collect::<Vec<_>>();

    while (ip as usize) < instructions_with_visitation.len() && !instructions_with_visitation[ip as usize].1 {
        instructions_with_visitation[ip as usize].1 = true;

        let (ip_increment, acc_increment) = match instructions_with_visitation[ip as usize].0 {
            Instruction::Acc(v) => (1, v),
            Instruction::Nop(_) => (1, 0),
            Instruction::Jmp(v) => (v, 0),
        };

        ip += ip_increment;
        acc += acc_increment;
    }

    let looped = (ip as usize) < instructions_with_visitation.len();

    (acc, looped)
}

pub fn problem1() -> Result<(), ParseError> {
    let instructions = parse_input()?;

    let (acc, _) = run(instructions);
    println!("8/1: Value of the accumulator before looping: {}", acc);

    Ok(())
}

fn flip(instruction: Instruction) -> Instruction {
    match instruction {
        Instruction::Acc(_) => instruction,
        Instruction::Jmp(v) => Instruction::Nop(v),
        Instruction::Nop(v) => Instruction::Jmp(v),
    }
}

pub fn problem2() -> Result<(), ParseError> {
    let instructions = parse_input()?;
    let mut acc = 0;

    for i in 0..instructions.len() {
        let mut copy = instructions.clone();
        copy[i] = flip(copy[i]);

        let (result, looped) = run(copy);
        if !looped {
            acc = result;
            break;
        }
    }

    println!("8/2: Value of the accumulator when not looping: {}", acc);

    Ok(())
}
