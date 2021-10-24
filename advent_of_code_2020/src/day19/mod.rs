use std::collections::HashMap;
use super::utils::ParseError;
use dynparser::{parse, rules_from_peg};

type Rules<'a> = Vec<&'a str>;
type Input<'a> = Vec<&'a str>;

fn parse_input() -> (Rules<'static>, Input<'static>) {
    let raw_input = include_str!("./data/input.txt");
    let rules_and_input = raw_input
        .split("\n\n")
        .collect::<Vec<_>>();

    let rules = rules_and_input[0]
        .lines()
        .filter(|v| *v != "")
        .collect::<Vec<_>>();

    let input = rules_and_input[1]
        .lines()
        .filter(|v| *v != "")
        .collect::<Vec<_>>();

    (rules, input)
}

fn convert_rules_to_peg(rules: Rules) -> String {
    let mut rules = rules;

    // dynparser expects one rule called 'main' as an entry point
    // Since rule '0' corresponds to this in our grammar, we just define one
    // additional rule:
    rules.push("main: 0");

    // the PEG parser expects a newline at the end of the string defining the
    // grammar...
    rules.push("");

    let ortrta: String = rules.join("\n");

    let ortrta = ortrta.replace("|", "\n  /");
    let ortrta = ortrta.replace(":", " =");
    let ortrta = ortrta.replace("\"a\"", r#"'a'"#);
    let ortrta = ortrta.replace("\"b\"", r#"'b'"#);

    ortrta
}

pub fn problem1() -> Result<(), ParseError> {
    let (rules, input) = parse_input();

    let prepared_rules = convert_rules_to_peg(rules);

    let peg_rules = rules_from_peg(&prepared_rules).unwrap();

    let result = input.iter()
        .map(|v| parse(v, &peg_rules).is_ok())
        .filter(|m| *m)
        .count();

    println!("19/1: # of successfully parsed input lines: {}", result);

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum Element {
    Expression(usize),
    Leaf(String),
}

fn parse_rhs(rhs: &str) -> Vec<Element> {
    rhs.split(" ")
        .map(|e| {
            if e.contains("a") {
                return Element::Leaf("a".to_owned());
            }

            if e.contains("b") {
                return Element::Leaf("b".to_owned());
            }

            Element::Expression(e.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>()
}

fn parse_tree(rules: Vec<&str>) -> HashMap<usize, Vec<Vec<Element>>> {
    let mut map = HashMap::new();

    for rule in &rules {
        let index_rule = rule.split(": ").collect::<Vec<_>>();
        let index = index_rule[0].parse::<usize>().unwrap();

        let rule = index_rule[1].split(" | ")
            .map(|v| parse_rhs(v))
            .collect::<Vec<_>>();

        map.entry(index).or_insert(rule);
    }

    map
}

fn test(s: &str, rules: &HashMap<usize, Vec<Vec<Element>>>, todo: Vec<usize>) -> bool {
    if todo.len() == 0 {
        return s.len() == 0;
    }

    let next = todo[0];
    let others: Vec<usize> = todo[1..].iter().cloned().collect();
    let descend = rules.get(&next).unwrap().clone();

    // we found a leaf ('a' or 'b')
    if descend.len() == 1 && descend[0].len() == 1 {
        if s.len() == 0 {
            return false;
        }
        let f = &descend[0][0];

        if &Element::Leaf(s[0..1].to_string()) == f {
            return test(&s[1..].to_string(), rules, others.clone());
        } else if let &Element::Leaf(_) = f {
            return false;
        }
    }

    descend.iter()
        .any(|v| {
            let mut follow_up = vec![];
            let mut first: Vec<usize> = v.iter()
                .cloned()
                .filter_map(|v| -> Option<usize> {
                    if let Element::Expression(e) = v {
                        Some(e)
                    } else {
                        None
                    }
                })
                .collect();

            let mut rest = others.clone();
            follow_up.append(&mut first);
            follow_up.append(&mut rest);
            test(s, rules, follow_up)
        })
}

pub fn problem2() -> Result<(), ParseError> {
    let (rules, input) = parse_input();

    let mut patched_rules = rules.into_iter()
        .filter(|r| !r.starts_with("8:") && !r.starts_with("11:"))
        .collect::<Vec<_>>();

    patched_rules.push("8: 42 | 42 8");
    patched_rules.push("11: 42 31 | 42 11 31");

    let tree = parse_tree(patched_rules);

    let main_rule: Vec<usize> = tree.get(&0).unwrap()[0].iter()
        .filter_map(|v| if let Element::Expression(e) = v {
            Some(e)
        } else {
            None
        })
        .cloned()
        .collect();

    let result = input.iter()
        .filter(|i| test(i, &tree, main_rule.clone()))
        .count();

    println!("19/2: # of successfully parsed input lines: {}", result);

    Ok(())
}
