use std::collections::HashSet;
use regex::Regex;
use super::utils::ParseError;

fn parse_content(content: &str) -> Result<(usize, &str), ParseError> {
    lazy_static!{
        static ref RE: Regex = Regex::new(r"^(\d+) (.*?) bags?$").unwrap();
    }
    let cap = RE.captures(content).ok_or(ParseError::new(&format!("Not valid content: {}", content)))?;
    let amount = cap[1].parse::<usize>()?;
    let description = cap.get(2).map(|m| m.as_str()).ok_or(ParseError::new(&format!("Not valid content: {}", content)))?;

    Ok((amount, description))
}

fn parse_rule(rule: &str) -> Result<(&str, Vec<(usize, &str)>), ParseError> {
    let mut rule_split: Vec<&str> = rule.split(" bags contain ").collect();

    let contents = rule_split.pop().ok_or(ParseError::new(&format!("Not a rule: '{}'", rule)))?;
    let bag = rule_split.pop().ok_or(ParseError::new(&format!("Not a rule: '{}'", rule)))?;

    if contents == "no other bags." {
        return Ok((bag, vec![]));
    }

    let mut contents_description: Vec<&str> = contents.split(", ").collect();
    let last_bag = contents_description.pop().ok_or(ParseError::new(&format!("Not a rule: '{}'", rule)))?;
    let last_bag = last_bag.split(".").next().ok_or(ParseError::new(&format!("Not a rule: '{}'", rule)))?;
    contents_description.push(last_bag);

    let contents = contents_description.iter().map(|v| parse_content(v)).collect::<Result<Vec<_>, ParseError>>()?;

    Ok((bag, contents))
}

fn parse_input() -> Result<Vec<(&'static str, Vec<(usize, &'static str)>)>, ParseError> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(parse_rule)
        .collect::<Result<Vec<_>, ParseError>>()
}

fn contains_bag(rule: &Vec<(usize, &str)>, search: &Vec<&str>) -> bool {
    rule.iter().any(|(_, bag)| search.contains(bag))
}

pub fn problem1() -> Result<(), ParseError> {
    let input = parse_input()?;

    let mut search = vec!["shiny gold"];
    let mut previous_result = 0;
    let mut relevant_bags: HashSet<&str> = HashSet::new();

    loop {
        let foo = input.iter()
            .filter(|(_, content)| contains_bag(&content, &search))
            .map(|(bag, _)| bag)
            .collect::<Vec<_>>();

        search = foo.iter().map(|v| **v).collect::<Vec<_>>();
        relevant_bags.extend(foo.into_iter().collect::<HashSet<_>>());

        if relevant_bags.len() == previous_result {
            break;
        }
        previous_result = relevant_bags.len();
    }

    let result = relevant_bags.len();

    println!("7/1: # of colors of bags that can contain at least one 'shiny gold': {}", result);

    Ok(())
}

fn count_bags(bag: &str, rules: &Vec<(&str, Vec<(usize, &str)>)>) -> Result<usize, ParseError> {
    let relevant_rules = rules.iter()
        .filter(|(b, _)| *b == bag)
        .collect::<Vec<_>>();

    if relevant_rules.len() != 1 {
        return Err(ParseError::new(&format!("No or more than one rule for bag {}", bag)));
    }

    let relevant_rule = relevant_rules[0];

    let number_of_bags: usize = relevant_rule.1.iter()
        .map(|(n, b)| count_bags(b, rules).map(|v| v * n))
        .collect::<Result<Vec<_>, ParseError>>()?
        .into_iter()
        .sum::<usize>() + 1;

    Ok(number_of_bags)
}

pub fn problem2() -> Result<(), ParseError> {
    let rules = parse_input()?;

    let result = count_bags(&"shiny gold", &rules)? - 1;

    println!("7/2: # of bags one 'shiny gold' bag contains: {}", result);

    Ok(())
}
