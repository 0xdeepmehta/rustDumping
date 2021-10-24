use std::collections::HashMap;
use super::utils::ParseError;

fn run(start: Vec<i32>, max: usize) -> Result<i32, ParseError> {
    let s = start.len();
    let mut m = HashMap::new();

    // don't store the last known index of the last number, this is done in the
    // main loop below. otherwise, we can't determine the second-to-last time
    // number was spoken which is what we actually need for the main loop.
    for (i, r) in start.iter().cloned().enumerate().take(start.len() - 1) {
        m.entry(r).or_insert(i);
    }

    let mut record: Vec<i32> = Vec::with_capacity(max + 2);
    let mut start = start;
    record.append(&mut start);

    for i in s..max {
        let last = *record.get(record.len() - 1).ok_or(ParseError::new("Empty record found."))?;
        let last_index = m.get(&last);

        if let Some(n) = last_index {
            let new = (i - *n - 1) as i32;
            record.push(new);
            m.entry(last).and_modify(|v| *v = i - 1);
        } else {
            record.push(0);
            m.entry(last).or_insert(i - 1);
        }
    }

    Ok(*record.last().unwrap())
}

pub fn problem1() -> Result<(), ParseError> {
    let input = vec![1,0,15,2,10,13];

    let result = run(input, 2020)?;
    println!("15/1: last number is {}", result);

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    let input = vec![1,0,15,2,10,13];

    let result = run(input, 30_000_000)?;
    println!("15/2: last number is {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
        let input = vec![0, 3, 6];
        assert_eq!(436, run(input, 2020).unwrap());
    }

    #[test]
    pub fn example_1_2() {
        let input = vec![2, 3, 1];
        assert_eq!(78, run(input, 2020).unwrap());
    }

    #[test]
    pub fn example_1_3() {
        let input = vec![3,1,2];
        assert_eq!(1836, run(input, 2020).unwrap());
    }
}