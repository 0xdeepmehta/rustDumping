use super::tile::Tile;

pub type Coords = (usize, usize);

fn find_pattern(image: &Tile, pattern: &Vec<Vec<char>>) -> Vec<Coords> {
    let size_image = image.data.len();
    let height_pattern = pattern.len();
    let width_pattern = pattern[0].len();

    let mut positions = vec![];
    for y in 0..(size_image-height_pattern) {
        for x in 0..(size_image-width_pattern) {
            let mut found = true;
            for my in 0..height_pattern {
                for mx in 0..width_pattern {
                    if pattern[my][mx] == '#' && image.data[y + my][x + mx] != '#' {
                        found = false;
                        break;
                    }
                }
                if !found {
                    break;
                }
            }

            if found {
                positions.push((x, y));
            }
        }
    }

    positions
}

pub fn transform_and_find_pattern(image: &Tile, pattern: &Vec<Vec<char>>) -> Option<(Tile, Vec<Coords>)> {
    for r in 0..4 {
        for f in vec![true, false] {
            let transformed = image.transform(r, f, false);
            let positions = find_pattern(&transformed, pattern);
            if !positions.is_empty() {
                return Some((transformed, positions));
            }
        }
    }

    None
}

pub fn remove_pattern(image: Tile, pattern: &Vec<Vec<char>>, positions: &Vec<Coords>) -> Tile {
    let height_pattern = pattern.len();
    let width_pattern = pattern[0].len();
    let mut image = image;

    for p in positions {
        for y in 0..height_pattern {
            for x in 0..width_pattern {
                if pattern[y][x] == '#' {
                    image.data[p.1 + y][p.0 + x] = 'O';
                }
            }
        }
    }
    image
}