use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use rayon::prelude::*;

const MAP_WIDTH: usize = 50;
const MAP_HEIGHT: usize = 50;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Tile {
    Ground,
    Tree,
    Lumberyard,
}

pub struct Map {
    current_state: u64,
    states: HashMap<u64, Vec<Tile>>,
    states_transforms: HashMap<u64, u64>,
    largest_state: u64,
}

impl Map {
    pub fn from_str(input: &str) -> Self {
        let mut tiles = Vec::with_capacity(MAP_WIDTH * MAP_HEIGHT);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile = match c {
                    '.' => Tile::Ground,
                    '|' => Tile::Tree,
                    '#' => Tile::Lumberyard,
                    _ => panic!("Unknown tile type: {}", c),
                };
                tiles.push(tile);
            }
        }

        let mut states = HashMap::new();
        states.insert(0, tiles);

        Self { current_state: 0, states, states_transforms: HashMap::new(), largest_state: 0 }
    }

    pub fn step(&mut self) {
        if self.states_transforms.contains_key(&self.current_state) {
            let transform = self.states_transforms.get(&self.current_state).unwrap();
            self.current_state = *transform;
            return;
        }

        let current_tiles = self.states.get(&self.current_state).unwrap();
        let new_tiles = current_tiles
            .iter()
            .enumerate()
            .map(|(index, tile)| {
                let x = index % MAP_WIDTH;
                let y = index / MAP_WIDTH;
                match tile {
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
                }
            })
            .collect::<Vec<_>>();

        let existing_state = self.states
            .iter()
            .find(|(state_num, state)| state == &&new_tiles);
        if let Some((state_num, _)) = existing_state {
            self.states_transforms.insert(self.current_state, *state_num);
            self.current_state = *state_num;
            return;
        }

        self.largest_state += 1;
        self.states.insert(self.largest_state, new_tiles.clone());
        self.states_transforms.insert(self.current_state, self.largest_state);
        self.current_state = self.largest_state;
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
        let index = y * MAP_WIDTH + x;
        let tiles = self.states.get(&self.current_state).unwrap();
        tiles[index]
    }

    pub fn count_tiles(&self, tile: Tile) -> usize {
        let tiles = self.states.get(&self.current_state).unwrap();
        tiles.iter().filter(|t| **t == tile).count()
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