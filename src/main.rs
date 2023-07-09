mod map;

const STEPS: usize = 1000000000;
const PRINT_STEPS: usize = 100;

fn main() {
    let input = include_str!("../input/input.txt");

    let mut map = map::Map::from_str(input);
    let mut last_time = std::time::Instant::now();
    for step in 0..STEPS {
        if step % PRINT_STEPS == 0 {
            print!(
                "Step {}/{} ({:.4}%), about {:.2} steps/sec. Est.: {:.0} minutes\r",
                step,
                STEPS,
                step as f32 / STEPS as f32 * 100.0,
                PRINT_STEPS as f64 / last_time.elapsed().as_secs_f64(),
                (STEPS as f64 - step as f64) / (PRINT_STEPS as f64 / last_time.elapsed().as_secs_f64()) / 60.0
            );
            last_time = std::time::Instant::now();
        }
        map.step();
    }
    println!();

    let trees = map.count_tiles(map::Tile::Tree);
    let lumberyards = map.count_tiles(map::Tile::Lumberyard);
    println!("Total resource value: {}", trees * lumberyards);
}
