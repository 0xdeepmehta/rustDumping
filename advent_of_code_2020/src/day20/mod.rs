use std::collections::HashMap;
use std::str::FromStr;

mod tile;
mod tilehash;
mod tileconnection;
mod patterns;
mod reconstruct;

use super::utils::ParseError;
use patterns::{transform_and_find_pattern, remove_pattern};
use reconstruct::reconstruct_image;
use tile::Tile;

fn parse_input() -> Result<Vec<Tile>, ParseError> {
    let input = include_str!("./data/input.txt");
    input
        .split("\n\n")
        .filter(|v| *v != "")
        .map(|v| Tile::from_str(v))
        .collect::<Result<Vec<_>, ParseError>>()
}

pub fn problem1() -> Result<(), ParseError> {
    let input = parse_input()?;

    let hashes = input.iter()
        .map(|t| t.hashes())
        .collect::<Vec<_>>();

    let result: u64 = hashes.iter()
        .map(|h| (h.id(), h.number_of_neighbors(&hashes)))
        .filter(|n| n.1 == 2)
        .map(|h| h.0)
        .product();

    println!("20/1: Product of the ids of all four corners of the map: {}", result);

    Ok(())
}

fn get_monster() -> Vec<Vec<char>> {
    include_str!("./data/monster.txt")
        .lines()
        .map(|v| v.chars().collect())
        .collect()
}

pub fn problem2() -> Result<(), ParseError> {
    let tiles = parse_input()?;

    let hashes = tiles.iter()
        .map(|t| t.hashes())
        .collect::<Vec<_>>();

    let relations = hashes.iter()
        .map(|h| (h.id(), h.find_neighbors(&hashes)))
        .collect::<Vec<_>>();

    // Both the example and my input have a corner that can be considered "top
    // left" without rotation or flipping the image.
    // "Top left" is defined as the tile that has two neighbors and the
    // neighbors are to the right and below the top left tile, i.e.
    //   my_border = [1, 2]
    let top_left = &relations.iter()
        // find corners
        .filter(|r| r.1.len() == 2)
        // find corner with neighbors to the right (my_border == 1) and bottom
        // (my_border == 2) of the corner
        .filter(|r| {
            let my_borders = r.1.iter().map(|v| v.my_border).collect::<Vec<_>>();
            my_borders.contains(&1) && my_borders.contains(&2)
        })
        .next().unwrap();

    let connections = relations.iter()
        .map(|v| &v.1)
        .cloned()
        .flatten()
        .collect::<Vec<_>>();

    let mut connections_map = HashMap::new();
    for c in &connections {
        connections_map.entry((c.id, c.my_border)).or_insert(c);
    }

    // construct the image based on the tile connections
    let image = reconstruct_image(&tiles, &connections, top_left.0);

    // rotate and flip image and look for monsters
    let monster = get_monster();
    if let Some((transformed, monsters)) = transform_and_find_pattern(&image, &monster) {
        let image_without_monsters = remove_pattern(transformed, &monster, &monsters);
        let result = image_without_monsters.count('#');
        println!("20/2: water roughness: {}", result);
    } else {
        println!("No monsters found!");
    }

    Ok(())
}
