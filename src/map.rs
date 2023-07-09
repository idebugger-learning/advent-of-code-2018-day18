use std::fmt::Display;

const MAP_WIDTH: usize = 50;
const MAP_HEIGHT: usize = 50;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

    pub fn step(&mut self) {
        let mut new_tiles = [Tile::Ground; MAP_WIDTH * MAP_HEIGHT];
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let tile = self.get_tile(x, y);
                let new_tile = match tile {
                    Tile::Ground => {
                        let (adjacent_trees, _) =
                            self.count_adjacent_tiles(x, y);
                        if adjacent_trees >= 3 {
                            Tile::Tree
                        } else {
                            Tile::Ground
                        }
                    }
                    Tile::Tree => {
                        let (_, adjacent_lumberyards) = self.count_adjacent_tiles(x, y);
                        if adjacent_lumberyards >= 3 {
                            Tile::Lumberyard
                        } else {
                            Tile::Tree
                        }
                    }
                    Tile::Lumberyard => {
                        let (adjacent_trees, adjacent_lumberyards) =
                            self.count_adjacent_tiles(x, y);
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
        self.tiles = new_tiles;
    }

    fn count_adjacent_tiles(&self, x: usize, y: usize) -> (usize, usize) {
        let mut count_trees = 0;
        let mut count_lumberyards = 0;

        let mut adjacent_tiles = [(0, 0); 8];
        let mut num = 0;
        if x > 0 {
            adjacent_tiles[num] = (x - 1, y);
            num += 1;
            if y > 0 {
                adjacent_tiles[num] = (x - 1, y - 1);
                num += 1;
            }
            if y < MAP_HEIGHT - 1 {
                adjacent_tiles[num] = (x - 1, y + 1);
                num += 1;
            }
        }
        if x < MAP_WIDTH - 1 {
            adjacent_tiles[num] = (x + 1, y);
            num += 1;
            if y > 0 {
                adjacent_tiles[num] = (x + 1, y - 1);
                num += 1;
            }
            if y < MAP_HEIGHT - 1 {
                adjacent_tiles[num] = (x + 1, y + 1);
                num += 1;
            }
        }
        if y > 0 {
            adjacent_tiles[num] = (x, y - 1);
            num += 1;
        }
        if y < MAP_HEIGHT - 1 {
            adjacent_tiles[num] = (x, y + 1);
            num += 1;
        }

        for (x, y) in adjacent_tiles[0..num].iter() {
            let tile = self.get_tile(*x, *y);
            match tile {
                Tile::Tree => count_trees += 1,
                Tile::Lumberyard => count_lumberyards += 1,
                _ => (),
            }
        }
        (count_trees, count_lumberyards)
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