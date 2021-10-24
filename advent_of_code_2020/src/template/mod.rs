use super::utils::ParseError;

fn parse_input() -> Vec<i32> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

pub fn problem1() -> Result<(), ParseError> {
    let input = parse_input();

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    let input = parse_input();

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
    }

    #[test]
    pub fn example_2_1() {
    }
}