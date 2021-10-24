use super::tileconnection::TileConnection;

#[derive(Debug)]
pub struct TileHash {
    id: u64,
    data: Vec<u64>,
}

impl TileHash {
    pub fn new(id: u64, hashes: Vec<u64>) -> Self {
        TileHash { id, data: hashes }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn flip(hash: u64) -> u64 {
        let mut flip = 0;
        for i in 0..10 {
            let bit = 1 << i;
            flip += if hash & bit != 0 { 1 } else { 0 } << (9 - i);
        }
        flip
    }

    pub fn number_of_neighbors(&self, all_hashes: &Vec<TileHash>) -> u64 {
        self.find_neighbors(all_hashes).len() as u64
    }

    pub fn find_neighbors(&self, all_hashes: &Vec<TileHash>) -> Vec<TileConnection> {
        let other_tiles = all_hashes.iter().filter(|t| t.id != self.id).collect::<Vec<_>>();

        // find all tiles that share a border with the given tile
        let find = |my_border: usize, my_border_hash: &u64, flipped: bool| {
            other_tiles.iter()
                .filter_map(|neighbor| {
                    let matching_borders = neighbor.data.iter().enumerate()
                        .filter_map(|(next_border, other_hash)| if other_hash == my_border_hash {
                            Some(TileConnection { id: self.id, my_border, next_tile: neighbor.id, next_border, flipped })
                        } else {
                            None
                        })
                        .collect::<Vec<_>>();

                        let number_of_shared_borders = matching_borders.len();

                    if number_of_shared_borders > 1 {
                        panic!(format!("Tile {} shares more than one border with tile {}", self.id, neighbor.id))
                    } else if number_of_shared_borders == 1 {
                        Some(matching_borders[0])
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        };

        let mut connections = vec![];
        for h in self.data.iter().enumerate() {
            let (my_border, my_border_hash) = h;
            let mut connection = find(my_border, my_border_hash, true);
            connections.append(&mut connection);
        }

        // do everything again but flip it
        for h in self.data.iter().map(|h| Self::flip(*h)).enumerate() {
            let (my_border, my_border_hash) = h;
            let mut connection = find(my_border, &my_border_hash, false);
            connections.append(&mut connection);
        }

        connections
    }

    fn print(&self) {
        println!("Tile {}:", self.id);
        for h in &self.data {
            print!("{}  ", h);
        }
        println!("");
    }
}
