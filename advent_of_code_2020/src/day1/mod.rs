use itertools::Itertools;

pub fn fold_2(v: &Vec<i32>) -> Option<i32> {
    for c in v.into_iter().combinations(2) {
        if c[0] + c[1] == 2020 {
            return Some(c[0] * c[1]);
        }
    }

    None
}

pub fn fold_3(v: &Vec<i32>) -> Option<i32> {
    for c in v.into_iter().combinations(3) {
        if c[0] + c[1] + c[2] == 2020 {
            return Some(c[0] * c[1] * c[2]);
        }
    }

    None
}

fn parse_input() -> Vec<i32> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

pub fn problem1() {
    let input = parse_input();
    let solution = fold_2(&input);

    if let Some(result) = solution {
        println!("1/1: {}", result);
    } else {
        println!("Found nothing.");
    }
}

pub fn problem2() {
    let input = parse_input();
    let solution = fold_3(&input);

    if let Some(result) = solution {
        println!("1/2: {}", result);
    } else {
        println!("Found nothing.");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_1_1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(Some(514579), fold_2(&input));
    }

    #[test]
    pub fn example_2_1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(Some(241861950), fold_3(&input));
    }
}