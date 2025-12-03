// #![no_std]
use crate::{play_tournament, update_strategies, Network, Strategy, DIM, NUM_VERTICES};
use minifb::{Key, Window, WindowOptions};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

const WIDTH: usize = DIM;
const HEIGHT: usize = DIM;

/// Maps a strategy to a 32-bit color value.
fn strategy_to_color(strategy: Strategy) -> u32 {
    match strategy {
        Strategy::Rock => 0x00FF0000,     // Red
        Strategy::Paper => 0x0000FF00,    // Green
        Strategy::Scissors => 0x000000FF, // Blue
    }
}

/// Calculates a blended color based on the historical average of strategies.
fn history_to_blended_color(history: &VecDeque<Strategy>) -> u32 {
    if history.is_empty() {
        return 0; // Black for no history
    }

    let mut rock_count = 0.0;
    let mut paper_count = 0.0;
    let mut scissors_count = 0.0;

    for strategy in history {
        match strategy {
            Strategy::Rock => rock_count += 1.0,
            Strategy::Paper => paper_count += 1.0,
            Strategy::Scissors => scissors_count += 1.0,
        }
    }

    let total = history.len() as f32;
    let r = (rock_count / total * 255.0) as u32;
    let g = (paper_count / total * 255.0) as u32;
    let b = (scissors_count / total * 255.0) as u32;

    // Combine into a 32-bit color value
    (r << 16) | (g << 8) | b
}

/// Runs the simulation and displays the strategy grid in a window.
pub fn run_simulation_with_visualization() {
    // --- Simulation Setup ---
    let mut strategies = [Strategy::Rock; NUM_VERTICES];
    let mut scores = [10; NUM_VERTICES];
    // Initialize with random strategies
    for i in 0..NUM_VERTICES {
        strategies[i] = match rand::random::<u8>() % 3 {
            0 => Strategy::Rock,
            1 => Strategy::Paper,
            _ => Strategy::Scissors,
        };
    }

    let network = Network::new();

    // --- History Tracking ---
    const HISTORY_LENGTH: usize = 40;
    let mut strategy_history: Vec<VecDeque<Strategy>> = vec![VecDeque::with_capacity(HISTORY_LENGTH); NUM_VERTICES];

    // --- Visualization Setup ---
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Rock Paper Scissors Simulation",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X8, // Scale up the 64x64 window to be more visible
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");

    // Limit to 60 frames per second to avoid burning CPU
    window.set_target_fps(60);

    // --- Simulation Timing ---
    let simulation_step_interval = Duration::from_millis(1); // e.g., run simulation step every 500ms
    let mut last_simulation_time = Instant::now();

    // --- Main Loop ---
    let mut o = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Only run the simulation logic if enough time has passed
        if last_simulation_time.elapsed() >= simulation_step_interval {
            // Run one iteration of the simulation
            play_tournament(&strategies, &mut scores, &network);
            update_strategies(&mut strategies, &scores, &network);
            println!("Simulation step {}", o);
            o += 1;

            // Update history for each cell
            for i in 0..NUM_VERTICES {
                if strategy_history[i].len() == HISTORY_LENGTH {
                    strategy_history[i].pop_front(); // Remove the oldest strategy
                }
                strategy_history[i].push_back(strategies[i]); // Add the new one
            }

            for (i, pixel) in buffer.iter_mut().enumerate() {
                *pixel = history_to_blended_color(&strategy_history[i]);
            }

            last_simulation_time = Instant::now();
        }

        // Update the window continuously.
        // We unwrap here as we want this to exit if it fails
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}