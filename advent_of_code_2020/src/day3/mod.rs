use super::utils::ParseError;

#[derive(PartialEq)]
enum Floor {
    Tree,
    Free,
}

fn parse_line(line: &str) -> Vec<Floor> {
    line.chars().map(|v| match v {
        '#' => Floor::Tree,
        _ => Floor::Free,
    })
    .collect::<Vec<_>>()
}

fn parse_input() -> Vec<Vec<Floor>> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(parse_line)
        .collect::<Vec<_>>()
}

fn get_floor_at_position<'a>((index, line): (usize, &'a Vec<Floor>), slope: &'a(usize, usize)) -> Option<&'a Floor> {
    line.iter()
        .cycle()
        .skip(index * slope.0 / slope.1)
        .next()
}

fn is_tree(floor: &Option<&Floor>) -> bool {
    if let Some(f) = floor {
        f == &&Floor::Tree
    } else {
        false
    }
}

fn count_trees_on_path(map: &Vec<Vec<Floor>>, slope: &(usize, usize)) -> u64 {
    map.iter()
        .enumerate()
        .step_by(slope.1)
        .map(|v| get_floor_at_position(v, slope))
        .filter(|t| is_tree(t))
        .count() as u64
}

pub fn problem1() -> Result<u64, ParseError> {
    let map = parse_input();

    let number_of_trees = count_trees_on_path(&map, &(3, 1));
    println!("3/1: # of trees: {}", number_of_trees);

    Ok(number_of_trees)
}

pub fn problem2() -> Result<u64, ParseError> {
    let map = parse_input();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let result: u64 = slopes.iter()
        .map(|slope| count_trees_on_path(&map, slope))
        .product();

    println!("3/2: product of # of trees on all slopes considered: {}", result);

    Ok(result)
}
