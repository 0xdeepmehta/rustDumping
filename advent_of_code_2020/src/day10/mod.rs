use super::utils::ParseError;

fn parse_input() -> Vec<i32> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn differences(v: &Vec<i32>) -> Vec<i32> {
    v
        .windows(2)
        .map(|v: &[i32]| v[1] - v[0])
        .collect::<Vec<_>>()
}

pub fn problem1() -> Result<(), ParseError> {
    let mut input = parse_input();

    // the airplane outlet
    input.push(0);
    input.sort_unstable();

    let input = input;
    let result = differences(&input);

    let ones = result.iter()
        .filter(|v| **v == 1)
        .count();
    let threes = result.iter()
        .filter(|v| **v == 3)
        .count() + 1;

    let result = ones * threes;
    println!("10/1: checksum is {}", result);

    Ok(())
}

// fortunately, we only have partition sizes <= 4 in the input
// so this is really easy to calculate...
fn combinations(n: &i32) -> i32 {
    match n {
        2 => 2,
        3 => 4,
        4 => 7,
        _ => 1,
    }
}

pub fn problem2() -> Result<(), ParseError> {
    let mut input = parse_input();
    input.push(0);
    input.sort_unstable();

    let diffs = differences(&input);

    let mut acc = 0;
    let mut partitions = vec![];
    for d in diffs {
        if d == 1 {
            acc += 1;
        } else if d == 3 {
            partitions.push(acc);
            acc = 0;
        }
    }
    partitions.push(acc);

    let result: usize = partitions.iter()
        .map(|v| combinations(v))
        .map(|v| v as usize)
        .product();

    println!("10/2: number of combinations is: {}", result);

    Ok(())
}
