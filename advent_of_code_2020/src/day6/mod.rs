use std::collections::HashSet;
use super::utils::ParseError;

fn parse_input() -> Vec<Vec<&'static str>> {
    let input = include_str!("./data/input.txt");
    input
        .split("\n\n")
        .filter(|v| *v != "")
        .map(|g| g.lines().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn answers_as_set(answers: &str) -> HashSet<char> {
    answers.chars().collect()
}

fn union(a: HashSet<char>, b: HashSet<char>) -> HashSet<char> {
    a.union(&b).cloned().collect()
}

fn intersection(a: HashSet<char>, b: HashSet<char>) -> HashSet<char> {
    a.intersection(&b).cloned().collect()
}

fn reduce_answers<F>(group: &Vec<&str>, set_operation: F) -> Option<usize>
    where
        F: FnMut(HashSet<char>, HashSet<char>) -> HashSet<char> {
    let mut answers = group.iter()
        .map(|g| answers_as_set(g));

    // We fetch the first element here and use it as the initial value for the
    // fold because one of the set operations applied here is the intersection.
    // If we start with an empty set, the intersection result will also be the
    // empty set.
    // There is a fold_first() that uses the first element of the iterator but
    // this is not in stable yet, only in nightly.
    // See https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold_first
    // See https://github.com/rust-lang/rust/issues/68125
    answers.next()
        .map(|initial| answers.fold(initial, set_operation))
        .map(|set| set.len())
}

pub fn problem1() -> Result<(), ParseError> {
    let groups = parse_input();

    let number_of_identical_answers: Option<usize> = groups.iter()
        .map(|g| reduce_answers(g, union))
        .sum();

    if let Some(v) = number_of_identical_answers{
        println!("6/1: # of answers: {}", v);
    } else {
        println!("6/1: Something went wrong.");
    }

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    let groups = parse_input();

    let number_of_answers: Option<usize> = groups.iter()
        .map(|g| reduce_answers(g, intersection))
        .sum();

    if let Some(v) = number_of_answers{
        println!("6/2: # of identical answers: {}", v);
    } else {
        println!("6/2: Something went wrong.");
    }

    Ok(())
}
