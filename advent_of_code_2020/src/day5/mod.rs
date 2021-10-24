use itertools::Itertools;
use super::utils::ParseError;

enum Direction {
    Up,
    Down,
}

fn char_to_direction(c: char) -> Direction {
    match c {
        'F' => Direction::Down,
        'L' => Direction::Down,
        _ => Direction::Up,
    }
}

fn parse_input() -> Vec<Vec<Direction>> {
    let input = include_str!("./data/input.txt");
    input
        .lines()
        .filter(|v| *v != "")
        .map(|v| v.chars()
            .map(|c| char_to_direction(c))
            .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn binary_search(partition_steps: &Vec<&Direction>) -> usize {
    let bits = partition_steps.len();
    let base: usize = 2;
    let initial_range = (0, base.pow(bits as u32));

    partition_steps.iter()
        .fold(initial_range, |acc, dir| {
            let half = (acc.1 - acc.0) / 2;
            match dir {
                Direction::Down => (acc.0, acc.1 - half),
                Direction::Up => (acc.0 + half, acc.1),
            }
        }).0
}

fn seat(pass: &Vec<Direction>) -> (usize, usize) {
    let row_encoded = pass.iter().take(7).collect::<Vec<_>>();
    let row = binary_search(&row_encoded);

    let col_encoded = pass.iter().skip(7).take(3).collect::<Vec<_>>();
    let col = binary_search(&col_encoded);

    (row, col)
}

fn seat_id(seat: (usize, usize)) -> usize {
    seat.0 * 8 + seat.1
}

pub fn problem1() -> Result<usize, ParseError> {
    let passes = parse_input();

    let mut seat_ids = passes.iter()
        .map(|p| seat(p))
        .map(|p| seat_id(p))
        .collect::<Vec<usize>>();
    seat_ids.sort_unstable();

    let highest_seat_id = seat_ids.iter().last().ok_or(ParseError::new("something went wrong"))?;

    println!("5/1: Highest seat id already taken: {}", highest_seat_id);

    Ok(*highest_seat_id)
}

pub fn problem2() -> Result<usize, ParseError> {
    let input = parse_input();

    let mut seat_ids = input.iter()
        .map(|p| seat(p))
        .map(|p| seat_id(p))
        .collect::<Vec<_>>();
    seat_ids.sort_unstable();

    let free_seats = seat_ids.iter()
        .tuple_windows::<(_, _)>()
        .map(|(&seat, &next_seat)| (next_seat - seat, seat))
        .filter(|(space, _)| *space == 2)
        .map(|(_, free_seat)| free_seat + 1)
        .collect::<Vec<_>>();

    let my_seat = free_seats.iter().next().ok_or(ParseError::new("something went wrong"))?;
    println!("5/2: My seat id: {}", my_seat);

    Ok(*my_seat)
}
