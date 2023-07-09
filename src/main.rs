mod map;

const STEPS: usize = 1000000000;

fn main() {
    let input = include_str!("../input/input.txt");

    let mut map = map::Map::from_str(input);
    let mut last_time = std::time::Instant::now();
    for step in 0..STEPS {
        if step % 100 == 0 {
            print!("Step {}/{} ({:.4}%), about {:.2} steps per second\r", step, STEPS, step as f32 / STEPS as f32 * 100.0, 100.0 / last_time.elapsed().as_secs_f64());
            last_time = std::time::Instant::now();
        }
        map.step();
    }
    println!();

    let trees = map.count_tiles(map::Tile::Tree);
    let lumberyards = map.count_tiles(map::Tile::Lumberyard);
    println!("Total resource value: {}", trees * lumberyards);
}
