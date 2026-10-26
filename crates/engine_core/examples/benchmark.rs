use engine_core::{CHUNK_SIZE, CellPos, MaterialId, World};
use std::time::{Duration, Instant};

const WIDTH: i32 = 256;
const HEIGHT: i32 = 256;
const STEPS: usize = 1000;
const WARMUP: usize = 100;
const BUDGET_MS: f64 = 1000.0 / 60.0;

fn build_world() -> World {
    let mut world = World::new();

    // Stone floor
    for x in 0..WIDTH {
        world.set(CellPos { x, y: HEIGHT - 1 }, MaterialId::STONE);
    }

    // Vertical obstacles with alternating gaps
    for x in (32..WIDTH - 16).step_by(32) {
        let gap_start = if (x / 32) % 2 == 0 { 80 } else { 160 };

        for y in 48..HEIGHT - 1 {
            if y < gap_start || y >= gap_start + 32 {
                world.set(CellPos { x, y }, MaterialId::STONE);
            }
        }
    }

    world
}

fn inject_sand(world: &mut World, target: usize, tick: usize) {
    let current = world
        .visible_cells()
        .iter()
        .filter(|cell| cell.material == MaterialId::SAND)
        .count();

    let mut remaining = target.saturating_sub(current);

    if remaining == 0 {
        return;
    }

    let offset = (tick % WIDTH as usize) as i32;

    for y in 0..HEIGHT - 1 {
        for i in 0..WIDTH {
            if remaining == 0 {
                return;
            }

            let x = (i + offset) % WIDTH;
            let pos = CellPos { x, y };

            if world.get(pos) == MaterialId::EMPTY {
                world.set(pos, MaterialId::SAND);
                remaining -= 1;
            }
        }
    }
}

fn percentile(sorted: &[f64], percent: f64) -> f64 {
    let index = ((sorted.len() - 1) as f64 * percent).ceil() as usize;
    sorted[index]
}

fn run_benchmark(target_sand: usize) {
    let mut world = build_world();

    println!("\n==================================");
    println!("Target sand cells: {}", target_sand);
    println!("World: {} x {}", WIDTH, HEIGHT);
    println!("Chunk size: {}", CHUNK_SIZE);
    println!("==================================");

    inject_sand(&mut world, target_sand, 0);

    // Warmup
    for tick in 0..WARMUP {
        world.step();
        inject_sand(&mut world, target_sand, tick);
    }

    // Separate measurements
    let mut total_durations = Vec::with_capacity(STEPS);
    let mut step_durations = Vec::with_capacity(STEPS);
    let mut inject_durations = Vec::with_capacity(STEPS);

    let wall_start = Instant::now();

    for tick in 0..STEPS {
        // Measure simulation
        let step_start = Instant::now();
        world.step();
        let step_ms = step_start.elapsed().as_secs_f64() * 1000.0;

        // Measure replenishment
        let inject_start = Instant::now();
        inject_sand(&mut world, target_sand, tick + WARMUP);
        let inject_ms = inject_start.elapsed().as_secs_f64() * 1000.0;

        let total_ms = step_ms + inject_ms;

        total_durations.push(total_ms);
        step_durations.push(step_ms);
        inject_durations.push(inject_ms);
    }

    let wall_elapsed: Duration = wall_start.elapsed();

    total_durations.sort_by(|a, b| a.total_cmp(b));
    step_durations.sort_by(|a, b| a.total_cmp(b));
    inject_durations.sort_by(|a, b| a.total_cmp(b));

    let average = total_durations.iter().sum::<f64>() / STEPS as f64;

    let p50 = percentile(&total_durations, 0.50);
    let p95 = percentile(&total_durations, 0.95);
    let p99 = percentile(&total_durations, 0.99);
    let worst = *total_durations.last().unwrap();

    let step_avg = step_durations.iter().sum::<f64>() / STEPS as f64;
    let inject_avg = inject_durations.iter().sum::<f64>() / STEPS as f64;

    let step_p99 = percentile(&step_durations, 0.99);
    let inject_p99 = percentile(&inject_durations, 0.99);

    let over_budget = total_durations.iter().filter(|&&ms| ms > BUDGET_MS).count();

    let actual_sand = world
        .visible_cells()
        .iter()
        .filter(|cell| cell.material == MaterialId::SAND)
        .count();

    println!("Actual final sand cells: {}", actual_sand);

    println!("\n--- COMBINED PERFORMANCE ---");
    println!("Average iteration: {:.3} ms", average);
    println!("Median (P50):      {:.3} ms", p50);
    println!("P95:               {:.3} ms", p95);
    println!("P99:               {:.3} ms", p99);
    println!("Worst iteration:   {:.3} ms", worst);
    println!("Over budget:       {}/{}", over_budget, STEPS);
    println!("Throughput:        {:.1} iterations/s", 1000.0 / average);

    println!("\n--- SIMULATION ONLY ---");
    println!("Average: {:.3} ms", step_avg);
    println!("P99:     {:.3} ms", step_p99);

    println!("\n--- SAND INJECTION ONLY ---");
    println!("Average: {:.3} ms", inject_avg);
    println!("P99:     {:.3} ms", inject_p99);

    println!("\n--- SUMMARY ---");
    println!("Wall time: {:.2} s", wall_elapsed.as_secs_f64());

    println!(
        "Combined 60 Hz budget: {}",
        if p99 <= BUDGET_MS { "PASS" } else { "FAIL" }
    );

    println!(
        "Simulation 60 Hz budget: {}",
        if step_p99 <= BUDGET_MS {
            "PASS"
        } else {
            "FAIL"
        }
    );
}

fn main() {
    println!("Pickle Engine — Stress Benchmark");
    println!("{} measured iterations per scenario", STEPS);

    run_benchmark(4_096);
    run_benchmark(16_384);
    run_benchmark(32_768);
    run_benchmark(49_152);
}
