use std::collections::HashMap;
use super::utils::ParseError;

fn parse_input() -> Vec<Vec<char>> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| v.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

type Coords = (i32, i32);
type World = HashMap<Coords, char>;

fn map_size(input: &Vec<Vec<char>>) -> Result<Coords, ParseError> {
    let height = input.len();
    let width = input.last().ok_or(ParseError::new("Empty map."))?.len();

    Ok((height as i32, width as i32))
}

fn generate_world(input: Vec<Vec<char>>) -> World {
    let mut map = HashMap::new();

    input.into_iter()
        .enumerate()
        .for_each(|(row, line)| {
            line.into_iter()
                .enumerate()
                .for_each(|(col, char)| {
                    map.entry((row as i32, col as i32)).or_insert(char);
                })
        });

    map
}

fn count_occupied_neighbors(map: &World, coords: &Coords) -> usize {
    let mut count = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            if let Some(status) = map.get(&(coords.0 + i, coords.1 + j)) {
                if status == &'#' {
                    count += 1;
                }
            }
        }
    }

    count
}

fn tick<F>(map: World, neighbor_count_strategy: F, neighbor_threshold: usize) -> World
    where F: Fn(&World, &Coords) -> usize {

    let mut new_world = HashMap::new();

    for (coords, status) in &map {
        let occupied_neighbors = neighbor_count_strategy(&map, &coords);

        if status == &'L' && occupied_neighbors == 0 {
            new_world.entry(*coords).or_insert('#');
        } else if status == &'#' && occupied_neighbors >= neighbor_threshold {
            new_world.entry(*coords).or_insert('L');
        } else {
            new_world.entry(*coords).or_insert(*status);
        }
    }

    new_world
}

fn count_occupied_seats(map: &World) -> usize {
    map.iter()
        .filter(|(_, &v)| v == '#')
        .count()
}

fn print_world(world: &World, size: &Coords) {
    for row in 0..size.0 {
        for col in 0..size.1 {
            print!("{}", world.get(&(row, col)).unwrap());
        }
        println!("");
    }
    println!("");
}

pub fn run<F>(neighbor_count_strategy: &F, neighbor_threshold: usize) -> Result<usize, ParseError>
    where F: Fn(&World, &Coords) -> usize {

    let input = parse_input();
    let _size = map_size(&input)?;
    let mut old_world = generate_world(input);

    // print_world(&old_world, &size);

    let mut last_count = 0;
    loop {
        let new_world = tick(old_world.clone(), neighbor_count_strategy, neighbor_threshold);
        // print_world(&new_world, &size);

        let occupied = count_occupied_seats(&new_world);
        if last_count == occupied {
            break;
        }
        old_world = new_world;
        last_count = occupied;
    }

    Ok(count_occupied_seats(&old_world))
}

pub fn problem1() -> Result<(), ParseError> {
    let result = run(&count_occupied_neighbors, 4)?;

    println!("11/1: # of occupied seats: {}", result);
    Ok(())
}

fn trace_occupation(map: &World, coords: &Coords, direction: &Coords) -> bool {
    let mut coords = (coords.0 + direction.0, coords.1 + direction.1);

    while let Some(status) = map.get(&coords) {
        if status == &'#' {
            return true;
        }

        if status == &'L' {
            return false;
        }

        coords.0 += direction.0;
        coords.1 += direction.1;
    }

    false
}

fn count_occupied_neighbors_2(world: &World, coords: &Coords) -> usize {
    let mut count = 0;

    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            if trace_occupation(world, coords, &(i, j)) {
                count += 1;
            }
        }
    }

    count
}

pub fn problem2() -> Result<(), ParseError> {
    let result = run(&count_occupied_neighbors_2, 5)?;

    println!("11/2: # of occupied seats: {}", result);
    Ok(())
}
