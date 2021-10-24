use std::collections::VecDeque;
use super::utils::ParseError;

type Deck = VecDeque<u64>;

#[derive(Clone)]
struct Game {
    player: Vec<Deck>,
    winner: Option<usize>,
}

fn parse_deck(s: &str) -> Deck {
    s.lines()
        .skip(1)
        .map(|l| l.parse::<u64>().unwrap())
        .collect()
}

fn parse_input() -> Vec<Deck> {
    let input = include_str!("./data/input.txt");
    input
        .split("\n\n")
        .filter(|v| *v != "")
        .map(|v| parse_deck(v))
        .collect::<Vec<_>>()
}

fn game_state_existed_before(previous_games: &Vec<Game>, current: &Game) -> bool {
    previous_games.iter()
        .any(|g| g.player[0] == current.player[0] && g.player[1] == current.player[1])
}

fn turn(mut game: Game, previous_turns: &mut Vec<Game>, recurse: bool, level: usize) -> Game {
    if game_state_existed_before(&previous_turns, &game) {
        game.winner = Some(0);
        return game;
    }

    let t1 = game.player[0].pop_front();
    let t2 = game.player[1].pop_front();

    if t1.is_none() {
        game.player[1].push_front(t2.unwrap());
        game.winner = Some(1);
        return game;
    }

    if t2.is_none() {
        game.player[0].push_front(t1.unwrap());
        game.winner = Some(0);
        return game;
    }

    let t1 = t1.unwrap();
    let t2 = t2.unwrap();
    let mut winner = 0;

    // determine sub game
    if recurse && t1 <= game.player[0].len() as u64 && t2 <= game.player[1].len() as u64 {
        let mut ng = game.clone();
        ng.player[0] = ng.player[0].iter().take(t1 as usize).cloned().collect::<VecDeque<_>>();
        ng.player[1] = ng.player[1].iter().take(t2 as usize).cloned().collect::<VecDeque<_>>();
        let rg = play_game(ng, recurse, level + 1);
        winner = rg.winner.unwrap();
    } else {
        if t2 > t1 {
            winner = 1;
        }
    }

    if winner == 0 {
        game.player[0].push_back(t1);
        game.player[0].push_back(t2);
    } else {
        game.player[1].push_back(t2);
        game.player[1].push_back(t1);
    }

    return game;
}

fn play_game(mut game: Game, recurse: bool, level: usize) -> Game {
    let mut pg = vec![];
    loop {
        let next_game = turn(game.clone(), &mut pg, recurse, level);
        pg.push(game);
        game = next_game;
        if game.winner.is_some() {
            return game;
        }
    }
}

pub fn problem1() -> Result<(), ParseError> {
    let decks = parse_input();
    let mut game = Game { player: decks, winner: None };

    loop {
        game = turn(game, &mut vec![], false, 1);
        if game.winner.is_some() {
            break;
        }
    }

    let winner = game.winner.unwrap();
    let score: u64 = game.player[winner].iter().rev().enumerate()
        .map(|(i, v)| (i + 1, v))
        .map(|(i, v)| (i as u64) * v)
        .sum();

    println!("22/1: score of winner's deck: {}", score);

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    let decks = parse_input();
    let mut game = Game { player: decks, winner: None };

    game = play_game(game, true, 1);

    let winner = game.winner.unwrap();
    let score: u64 = game.player[winner].iter().rev().enumerate()
        .map(|(i, v)| (i + 1, v))
        .map(|(i, v)| (i as u64) * v)
        .sum();

    println!("22/2: score of winner's deck: {}", score);

    Ok(())
}
