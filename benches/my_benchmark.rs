use criterion::{Criterion, criterion_group, criterion_main};
use rps::*;
use std::hint::black_box;

fn addition_benchmark(c: &mut Criterion) {
    c.bench_function("addition", |b| {
        b.iter(|| black_box(20.0) + black_box(123.1234253))
    });
}

fn rps_benchmark(c: &mut Criterion) {
    let mut agents = Vec::new();
    for _ in 0..NUM_VERTICES {
        let random_strategy = match rand::random::<u32>() % 3 {
            0 => Strategy::Rock,
            1 => Strategy::Paper,
            2 => Strategy::Scissors,
            _ => panic!("Invalid random number"),
        };
        agents.push(Agent {
            strategy: random_strategy,
            score: 0,
        });
    }
    let mut matrix = AdjacencyMatrix::new();
    for i in 0..NUM_VERTICES {
        matrix.add_edge(i, (i + 1) % NUM_VERTICES);
    }
    c.bench_function("rps", |b| {
        b.iter(|| {
            let new_agents = play_tournament(&matrix, agents.clone());
            black_box(new_agents);
        })
    });
}

criterion_group!(benches, rps_benchmark, addition_benchmark);
criterion_main!(benches);
