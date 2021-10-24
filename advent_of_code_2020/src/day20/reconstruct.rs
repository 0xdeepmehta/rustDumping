use std::collections::HashSet;
use std::collections::HashMap;
use super::tileconnection::TileConnection;
use super::tile::Tile;

fn find_next_tile(tile: u64, border: usize, connections: &Vec<TileConnection>) -> Option<&TileConnection> {
    connections.iter()
        .find(|c| c.next_tile == tile && c.next_border == border)
}

fn find_right_border(tile: u64, is_even: bool, connections: &Vec<TileConnection>) -> usize {
    connections.iter()
        .filter(|c| c.id == tile)
        .map(|c| c.my_border)
        // We are looking for the right border of a tile at the left border of
        // the image. Since it is a border tile, there are only three
        // neighbors. If the tile is not rotated, there will be two neighbors
        // in even directions (0 = up, 2 = down). the same is true if the tile
        // is rotated 180 degrees. If it is rotated by 90/270 degrees, the
        // border to the right will be odd.
        .filter(|b| b % 2 == if is_even { 0 } else { 1 })
        .next()
        .unwrap()
}

fn rotation_from_exit_right(exit: usize) -> usize {
    match exit {
        1 => 0,
        2 => 3,
        3 => 2,
        0 => 1,
        _ => panic!(format!("cannot map exit {} to any rotation", exit)),
    }
}

fn rotation_from_exit_down(exit: usize) -> usize {
    match exit {
        1 => 1,
        2 => 0,
        3 => 3,
        0 => 2,
        _ => panic!(format!("cannot map exit {} to any rotation", exit)),
    }
}

pub fn reconstruct_image(tiles: &Vec<Tile>, connections: &Vec<TileConnection>, top_left: u64) -> Tile {
    let size = (tiles.len() as f32).sqrt() as usize;

    let mut current_y_tile = top_left;
    let mut current_y_border = 2;
    let mut image = vec![];
    let mut is_x_border_even = false;
    let mut y_flip = false;
    let mut x_flip = false;
    for _y in 0..size {
        // construct a line
        let mut current_tile = current_y_tile;
        let mut current_border = find_right_border(current_tile, is_x_border_even, &connections);
        let mut line = vec![(current_tile, rotation_from_exit_down(current_y_border), false, x_flip)];
        for _x in 0..size - 1 {
            if let Some(next) = find_next_tile(current_tile, current_border, &connections) {
                current_tile = next.id;
                current_border = (next.my_border + 2) % 4;
                if next.flipped {
                    y_flip = !y_flip;
                }
                line.push((current_tile, rotation_from_exit_right(current_border), y_flip, false));
            }
        }
        image.push(line);

        if let Some(next_y) = find_next_tile(current_y_tile, current_y_border, &connections) {
            current_y_tile = next_y.id;
            current_y_border = (next_y.my_border + 2) % 4;
            if next_y.flipped {
                x_flip = !x_flip;
            }
            y_flip = x_flip;
            is_x_border_even = current_y_border % 2 != 0;

        }
    }

    // generate a hashmap for easier id based lookup of tiles
    let mut tile_map = HashMap::new();
    for tile in tiles {
        tile_map.entry(tile.id()).or_insert(tile);
    }

    // reconstruct image
    let mut image_data = vec![];
    let mut printed_tiles = HashSet::new();
    for l in &image {
        for y in 0..8 {
            let mut line = vec![];
            for (tile_id, rotation, y_flipped, x_flipped) in l.iter() {
                printed_tiles.insert((*tile_id, *rotation, *y_flipped, *x_flipped));
                let tile = tile_map.get(&tile_id).unwrap();
                let mut tile_line = tile.get_line_without_border(y, *rotation, *y_flipped, *x_flipped);
                line.append(&mut tile_line);
            }
            image_data.push(line);
        }
    }

    Tile::new(0, image_data)
}