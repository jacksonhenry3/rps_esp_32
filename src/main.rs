// #![no_std]

use rps::*;
mod visualization;

fn main() {
    // The original main loop has been moved into the visualization module
    // to run the simulation and render it frame-by-frame.
    visualization::run_simulation_with_visualization();
}
