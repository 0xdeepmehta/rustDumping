use std::hash::Hasher;
use std::hash::Hash;
use std::collections::HashSet;
use regex::Regex;
use super::utils::ParseError;

#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
}

impl Range {
    fn is_valid(&self, i: u32) -> bool {
        self.min <= i && i <= self.max
    }
}

#[derive(Debug)]
struct Rule {
    description: String,
    ranges: Vec<Range>,
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.description == other.description
    }
}

impl Eq for Rule {}

impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.description.hash(state);
    }
}

impl Rule {
    fn is_valid(&self, i: u32) -> bool {
        self.ranges.iter().any(|r| r.is_valid(i))
    }
}

type Ticket = Vec<u32>;

#[derive(Debug)]
struct Puzzle {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn read_rule(s: &str) -> Rule {
    lazy_static!{
        static ref RE: Regex = Regex::new(r"^(.+?): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    }

    let cap = RE.captures(s).unwrap();
    let description = cap[1].to_string();

    let mut ranges = vec![];

    let min = cap[2].parse::<u32>().unwrap();
    let max = cap[3].parse::<u32>().unwrap();
    ranges.push(Range { min, max });

    let min = cap[4].parse::<u32>().unwrap();
    let max = cap[5].parse::<u32>().unwrap();
    ranges.push(Range { min, max });

    Rule { description, ranges }
}

fn read_ticket(s: &str) -> Ticket {
    s.split(',')
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn parse_input() -> Puzzle {
    let input = include_str!("./data/input.txt");
    let puzzle = input
        .split("\n\n")
        .filter(|v| *v != "")
        .collect::<Vec<_>>();

    let rules = puzzle[0].lines()
        .filter(|v| !v.is_empty())
        .map(|l| read_rule(l))
        .collect::<Vec<_>>();

    let my_ticket = puzzle[1].lines()
        .skip(1)
        .map(|t| read_ticket(t))
        .next().unwrap();

    let nearby_tickets = puzzle[2].lines()
        .skip(1)
        .filter(|v| !v.is_empty())
        .map(|l| read_ticket(l))
        .collect::<Vec<_>>();

    Puzzle { rules, my_ticket, nearby_tickets }
}

fn is_valid_for_some_field(rules: &Vec<Rule>, n: u32) -> bool {
    rules.iter()
        .any(|r| r.is_valid(n))
}

fn find_invalid_fields(rules: &Vec<Rule>, ticket: &Ticket) -> Vec<u32> {
    ticket.iter()
        .filter(|n| !is_valid_for_some_field(rules, **n))
        .cloned()
        .collect::<Vec<_>>()
}

pub fn problem1() -> Result<(), ParseError> {
    let input = parse_input();

    let mut invalid_fields = vec![];
    for t in &input.nearby_tickets {
        let mut invalid_fields_for_ticket = find_invalid_fields(&input.rules, t);
        invalid_fields.append(&mut invalid_fields_for_ticket);
    }

    let result: u32 = invalid_fields.iter().sum();
    println!("16/1: sum of invalid fields is {}", result);

    Ok(())
}

fn is_valid(rules: &Vec<Rule>, ticket: &Ticket) -> bool {
    ticket.iter()
        .all(|n| is_valid_for_some_field(rules, *n))
}

fn determine_valid_rules(rules: &Vec<Rule>, n: u32) -> HashSet<&Rule> {
    rules.iter()
        .filter(|r| r.is_valid(n))
        .collect::<HashSet<_>>()
}

fn determine_valid_rules_2(rules: &Vec<Rule>, n: u32) -> HashSet<usize> {
    rules.iter()
        .enumerate()
        .filter(|(_, r)| r.is_valid(n))
        .map(|(i, _)| i)
        .collect::<HashSet<_>>()
}

pub fn problem2() -> Result<(), ParseError> {
    let input = parse_input();

    let valid_nearby_tickets = input.nearby_tickets.iter()
        .filter(|t| is_valid(&input.rules, t))
        .collect::<Vec<_>>();

    let len = input.rules.len();
    let mut candidates = vec![];

    for i in 0..len {

        let valid_rules = valid_nearby_tickets.iter()
            .map(|t| determine_valid_rules(&input.rules, t[i]))
            .collect::<Vec<_>>();

        let mut rules_iter = valid_rules.iter();
        let mut one_rule = rules_iter.next().cloned().unwrap();

        for r in rules_iter {
            one_rule = one_rule.intersection(&r).cloned().collect();
        }

        candidates.push((i, one_rule));
    }

    candidates.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
    let mut already_assigned = HashSet::new();

    let mut columns = vec![];
    for c in &mut candidates {
        for a in &already_assigned {
            c.1.remove(a);
        }

        if c.1.len() != 1 {
            panic!("could not uniquely determine which column corresponds to which field");
        }

        let next_column = c.1.iter().next().unwrap();
        columns.push((c.0, next_column));
        already_assigned.insert(*next_column);
    }

    let readable_candidates = columns.iter().map(|(i, c)| (i, c.description.clone())).collect::<Vec<_>>();

    let result: u64 = readable_candidates.iter()
        .filter(|(_, d)| d.contains("departure"))
        .map(|(i, _)| input.my_ticket[**i] as u64)
        .product();
    println!("16/2: sum of invalid fields is {}", result);

    Ok(())
}
