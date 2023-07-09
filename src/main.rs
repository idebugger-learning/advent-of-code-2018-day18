mod map;

fn main() {
    let input = include_str!("../input/input.txt");

    let mut map = map::Map::from_str(input);
    for _ in 0..10 {
        map = map.step();
    }

    let trees = map.count_tiles(map::Tile::Tree);
    let lumberyards = map.count_tiles(map::Tile::Lumberyard);
    println!("Total resource value: {}", trees * lumberyards);
}
