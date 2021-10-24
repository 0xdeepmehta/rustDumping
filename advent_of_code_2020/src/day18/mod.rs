use std::collections::VecDeque;
use super::utils::ParseError;

fn load_input() -> Vec<&'static str> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .collect::<Vec<_>>()
}

fn evaluate(values: &mut VecDeque<u64>, ops: &mut VecDeque<char>) -> u64 {
    while !ops.is_empty() {
        let op = ops.pop_front();
        let v1 = values.pop_front().unwrap();
        let v2 = values.pop_front().unwrap();

        match op {
            Some('+') => values.push_front(v1 + v2),
            Some('*') => values.push_front(v1 * v2),
            _ => panic!("Unknown operator {}", op.unwrap()),
        }
    }

    values.pop_front().unwrap()
}

fn evaluate_with_precedence(values: &mut VecDeque<u64>, ops: &mut VecDeque<char>) -> u64 {
    let mut mult_ops = VecDeque::new();
    let mut mult_vals = VecDeque::new();

    while ops.contains(&'+') {
        let op = ops.pop_front();

        if op.is_none() {
            let v = values.pop_front().unwrap();
            mult_vals.push_back(v);
            break;
        }

        let op = op.unwrap();
        if op == '+' {
            let v1 = values.pop_front().unwrap();
            let v2 = values.pop_front().unwrap();
            values.push_front(v1 + v2);
        }

        if op == '*' {
            let v1 = values.pop_front().unwrap();
            mult_vals.push_back(v1);
            mult_ops.push_back('*');
        }
    }

    mult_vals.append(values);
    mult_ops.append(ops);

    // exit early, if there are only + operations in our expression
    // because if that's the case, we have our result in mult_vals already
    if mult_ops.is_empty() {
        let result = mult_vals.pop_front().unwrap();
        return result;
    }

    let mut ops = mult_ops;
    let mut values = mult_vals;

    evaluate(&mut values, &mut ops)
}

fn parse<F>(s: &str, pos: usize, evaluator: &F) -> (usize, u64)
    where F: Fn(&mut VecDeque<u64>, &mut VecDeque<char>) -> u64 {

        let mut stack = VecDeque::new();
    let mut ops = VecDeque::new();

    // for (p, c) in s.chars().enumerate().skip(pos) {
    let chars = s.chars().collect::<Vec<_>>();
    let mut i = pos;
    while i < chars.len() {
        let (p, c) = (i, chars[i]);
        i += 1;
        match c {
            ' ' => {},
            '+' | '*' => ops.push_back(c),
            '0' => stack.push_back(0),
            '1' => stack.push_back(1),
            '2' => stack.push_back(2),
            '3' => stack.push_back(3),
            '4' => stack.push_back(4),
            '5' => stack.push_back(5),
            '6' => stack.push_back(6),
            '7' => stack.push_back(7),
            '8' => stack.push_back(8),
            '9' => stack.push_back(9),
            '(' => {
                let (s, v) = parse(s, p + 1, evaluator);
                stack.push_back(v);
                i += s;
            },
            ')' => {
                return (i - pos, evaluator(&mut stack, &mut ops));
            },
            _ => {
                panic!("Unexpected char: {}", c);
            }
        }
    }

    (0, evaluator(&mut stack, &mut ops))
}

pub fn problem1() -> Result<(), ParseError> {
    let input = load_input();

    let result: u64 = input.iter()
        .map(|s| parse(s, 0, &evaluate).1)
        .sum();
    println!("18/1: sum over all expressions is {}", result);

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    let input = load_input();

    let result: u64 = input.iter()
        .map(|s| parse(s, 0, &evaluate_with_precedence).1)
        .sum();
    println!("18/2: sum over all expressions is {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(71, parse(input, 0, &evaluate).1);
    }

    #[test]
    pub fn example_1_2() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(51, parse(input, 0, &evaluate).1);
    }

    #[test]
    pub fn example_1_3() {
        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(12240, parse(input, 0, &evaluate).1);
    }

    #[test]
    pub fn example_2_1() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        assert_eq!(231, parse(input, 0, &evaluate_with_precedence).1);
    }

    #[test]
    pub fn example_2_2() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        assert_eq!(51, parse(input, 0, &evaluate_with_precedence).1);
    }

    #[test]
    pub fn example_2_3() {
        let input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        assert_eq!(669060, parse(input, 0, &evaluate_with_precedence).1);
    }
}