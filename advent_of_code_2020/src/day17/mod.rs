use std::collections::HashMap;
use super::utils::ParseError;

type Coords = (i64, i64, i64, i64);
type World = HashMap<Coords, char>;

fn parse_input() -> World {
    let input = include_str!("./data/input.txt");
    let v: Vec<Vec<char>> = input
        .lines()
        .filter(|v| *v != "")
        .map(|v| v.chars().collect())
        .collect::<Vec<_>>();

    let mut map = HashMap::new();

    let z = 0;
    let w = 0;
    let mut y = 0;
    for line in v {
        let mut x = 0;
        for c in line {
            map.entry((x, y, z, w)).or_insert(c);
            x += 1;
        }
        y += 1;
    }

    map
}

fn count_neighbors(world: &World, coords: &Coords) -> i64 {
    let mut count: i64 = 0;
    for dw in -1..=1 {
        for dz in -1..=1 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                        continue;
                    }

                    if let Some(v) = world.get(&(coords.0 + dx, coords.1 + dy, coords.2 + dz, coords.3 + dw)) {
                        if v == &'#' {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

fn size(world: &World) -> (Coords, Coords) {
    let mut min = (1000, 1000, 1000, 1000);
    let mut max = (-1000, -1000, -1000, -1000);

    for (c, _) in world {
        if c.0 < min.0 {
            min.0 = c.0;
        }

        if c.1 < min.1 {
            min.1 = c.1;
        }

        if c.2 < min.2 {
            min.2 = c.2;
        }

        if c.3 < min.3 {
            min.3 = c.3;
        }

        if c.0 > max.0 {
            max.0 = c.0;
        }

        if c.1 > max.1 {
            max.1 = c.1;
        }

        if c.2 > max.2 {
            max.2 = c.2;
        }

        if c.3 > max.3 {
            max.3 = c.3;
        }
    }

    (min, max)
}

fn grow(world: &mut World) {
    let size = size(world);

    let min = (size.0.0 - 1, size.0.1 - 1, size.0.2 - 1, size.0.3 - 1);
    let max = (size.1.0 + 1, size.1.1 + 1, size.1.2 + 1, size.1.3 + 1);

    for w in min.3..=max.3 {
        for z in min.2..=max.2 {
            for y in min.1..=max.1 {
                for x in min.0..=max.0 {
                    world.entry((x, y, z, w)).or_insert('.');
                }
            }
        }
    }
}

fn tick(world: &mut World) {
    let old_world = world.clone();

    grow(world);
    for (c, s) in world {
        let alive = count_neighbors(&old_world, c);
        let is_alive = s == &'#';
        if is_alive && (alive == 2 || alive == 3) {
            *s = '#';
        } else if !is_alive && alive == 3 {
            *s = '#';
        } else {
            *s = '.';
        }
    }
}

fn count_alive(world: &World) -> u64 {
    let mut count = 0;
    for (_, s) in world {
        if s == &'#' {
            count += 1;
        }
    }
    count
}

fn print_world(world: &World) {
    let size = size(world);

    for w in size.0.3..=size.1.3 {
        for z in size.0.2..=size.1.2 {
            println!("z = {}, w = {}", z, w);
            for y in size.0.1..=size.1.1 {
                for x in size.0.0..=size.1.0 {
                    if let Some(c) = world.get(&(x, y, z, w)) {
                        if c == &'#' {
                            print!("#");
                        } else {
                            print!(".");
                        }
                    }
                }
                println!("");
            }
            println!("");
        }
    }
}

pub fn problem1() -> Result<(), ParseError> {
    let mut world = parse_input();

    for _ in 0..6 {
        tick(&mut world);
    }
    let result = count_alive(&world);
    // print_world(&world);

    println!("result: {}", result);

    Ok(())
}

pub fn problem2() -> Result<(), ParseError> {
    Ok(())
}
