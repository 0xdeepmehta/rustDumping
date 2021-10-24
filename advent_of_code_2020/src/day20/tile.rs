use regex::Regex;
use std::str::FromStr;

use super::tilehash::TileHash;
use crate::utils::ParseError;

#[derive(Debug)]
pub struct Tile {
    id: u64,
    pub data: Vec<Vec<char>>,
}

impl FromStr for Tile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
        }

        let id_str = s.lines().take(1).next().ok_or(ParseError::new(&format!("Could not find tile id in {}", s)))?;

        let cap = RE.captures(id_str).ok_or(ParseError::new(&format!("Could not extract id from tile header: {}", id_str)))?;
        let id = cap[1].parse::<u64>()?;
        let data = s.lines().skip(1).map(|v| v.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

        // remove the inner stuff from the tiles for better debugging
        // let mut data = data;
        // for y in 1..(data.len()-1) {
        //     for x in 1..(data[y].len()-1) {
        //         data[y][x] = ' ';
        //     }
        // }
        Ok(Self { id, data })
    }
}

impl Tile {
    pub fn new(id: u64, data: Vec<Vec<char>>) -> Self {
        Tile { id, data }
    }

    pub fn print(&self) {
        println!("Tile {}:", self.id);
        for line in &self.data {
            for c in line {
                print!("{}", c);
            }
            println!();
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn count(&self, needle: char) -> usize {
        self.data.iter()
            .map(|l| l.iter().filter(|c| **c == needle).count())
            .sum()
    }

    pub fn transform(&self, rotate: usize, flip_y: bool, flip_x: bool) -> Self {
        let flipped_data = (0..self.data.len())
            .map(|l| self.get_line(l, rotate, flip_y, flip_x))
            .collect::<Vec<_>>();
        Tile { id: 0, data: flipped_data }
    }

    fn hash_line(line: &Vec<char>) -> u64 {
        line.iter().enumerate()
            .fold(0, |acc, (i, c)| {
                acc + if c == &'#' { 1u64 << i as u64 } else { 0 }
            })
    }

    fn get_column(&self, column: usize) -> Vec<char> {
        self.data.iter().map(|v| v[column]).collect()
    }

    fn get_row(&self, row: usize) -> Vec<char> {
        self.data[row].clone()
    }

    pub fn hashes(&self) -> TileHash {
        let top = Self::hash_line(&self.get_row(0));
        let right = Self::hash_line(&self.get_column(9));

        // By flipping the next two hashes we make the hashes of the tile
        // rotation invariant
        let bottom = TileHash::flip(Self::hash_line(&self.get_row(9)));
        let left = TileHash::flip(Self::hash_line(&self.get_column(0)));

        TileHash::new(self.id, vec![top, right, bottom, left])
    }

    pub fn get_line(&self, line: usize, rotation: usize, y_flipped: bool, x_flipped: bool) -> Vec<char> {
        let line = if y_flipped { self.data.len() - line - 1 } else { line };

        // mighty inefficient, but it's at least somewhat recognizable what is
        // happening in here.
        let rotated_line = if rotation == 0 {
            self.data[line].clone()
        } else if rotation == 1 {
            self.data.iter().map(|v| v[line]).rev().collect::<Vec<_>>()
        } else if rotation == 2 {
            self.data[self.data.len() - line - 1].iter().rev().cloned().collect::<Vec<_>>()
        } else if rotation == 3 {
            self.data.iter().map(|v| v[self.data.len() - line - 1]).collect::<Vec<_>>()
        } else {
            panic!(format!("Unknown rotation: {}", rotation));
        };

        let flipped_line = if x_flipped {
            rotated_line.iter().rev().cloned().collect::<Vec<_>>()
        } else {
            rotated_line
        };

        flipped_line
    }

    pub fn get_line_without_border(&self, line: usize, rotation: usize, y_flipped: bool, x_flipped: bool) -> Vec<char> {
        let line = line + 1;
        let line = self.get_line(line, rotation, y_flipped, x_flipped);

        line.iter().skip(1).take(8).cloned().collect()
    }
}
