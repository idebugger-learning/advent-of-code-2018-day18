use std::fmt::Display;

const MAP_WIDTH: usize = 50;
const MAP_HEIGHT: usize = 50;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Ground,
    Tree,
    Lumberyard,
}

pub struct Map {
    tiles: [Tile; MAP_WIDTH * MAP_HEIGHT],
}

impl Map {
    pub fn from_str(input: &str) -> Self {
        let mut tiles = [Tile::Ground; MAP_WIDTH * MAP_HEIGHT];
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile = match c {
                    '.' => Tile::Ground,
                    '|' => Tile::Tree,
                    '#' => Tile::Lumberyard,
                    _ => panic!("Unknown tile type: {}", c),
                };
                tiles[y * MAP_WIDTH + x] = tile;
            }
        }
        Self { tiles }
    }

    pub fn step(&self) -> Self {
        let mut new_tiles = [Tile::Ground; MAP_WIDTH * MAP_HEIGHT];
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let tile = self.get_tile(x, y);
                let new_tile = match tile {
                    Tile::Ground => {
                        let adjacent_trees = self.count_adjacent_tiles(x, y, Tile::Tree);
                        if adjacent_trees >= 3 {
                            Tile::Tree
                        } else {
                            Tile::Ground
                        }
                    }
                    Tile::Tree => {
                        let adjacent_lumberyards = self.count_adjacent_tiles(x, y, Tile::Lumberyard);
                        if adjacent_lumberyards >= 3 {
                            Tile::Lumberyard
                        } else {
                            Tile::Tree
                        }
                    }
                    Tile::Lumberyard => {
                        let adjacent_lumberyards = self.count_adjacent_tiles(x, y, Tile::Lumberyard);
                        let adjacent_trees = self.count_adjacent_tiles(x, y, Tile::Tree);
                        if adjacent_lumberyards >= 1 && adjacent_trees >= 1 {
                            Tile::Lumberyard
                        } else {
                            Tile::Ground
                        }
                    }
                };
                new_tiles[y * MAP_WIDTH + x] = new_tile;
            }
        }
        Self { tiles: new_tiles }
    }

    fn count_adjacent_tiles(&self, x: usize, y: usize, target_tile: Tile) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let x = x as isize + dx;
                let y = y as isize + dy;
                if x < 0 || x >= MAP_WIDTH as isize || y < 0 || y >= MAP_HEIGHT as isize {
                    continue;
                }
                let tile = self.get_tile(x as usize, y as usize);
                if tile == target_tile {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[y * MAP_WIDTH + x]
    }

    pub fn count_tiles(&self, tile: Tile) -> usize {
        self.tiles.iter().filter(|&&t| t == tile).count()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let tile = self.get_tile(x, y);
                let c = match tile {
                    Tile::Ground => '🟫',
                    Tile::Tree => '🌲',
                    Tile::Lumberyard => '🪓',
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}