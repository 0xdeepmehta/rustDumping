#[derive(Debug, Clone, Copy)]
pub struct TileConnection {
    pub id: u64,
    pub my_border: usize,
    pub next_tile: u64,
    pub next_border: usize,
    pub flipped: bool,
}
