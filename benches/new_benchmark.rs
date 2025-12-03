#[macro_use]
extern crate criterion;
use criterion::{Criterion, black_box};
use rand::prelude::*;
use rps::*; // Replace with your crate name
use core::array;


pub const NUM_VERTICES: usize = 64 * 64;

// Helper to generate a random vector of strategies.
fn random_strategies() -> [Strategy;NUM_VERTICES] {
    let mut rng = rand::rng();
   
    array::from_fn(|_| match rng.random_range(0..3) {
            0 => Strategy::Rock,
            1 => Strategy::Paper,
            _ => Strategy::Scissors,
        })
}

// Helper to generate a random vector of scores.
fn random_scores() -> [i32;NUM_VERTICES] {
    let mut rng = rand::rng();
    array::from_fn(|_| rng.random_range(0..100))
    
}

fn bench_exp(c: &mut Criterion) {
    c.bench_function("exp in range", |b| {
        b.iter(|| {
            for n in -1000..=1000 {
                black_box(exp(n));
            }
        })
    });
    c.bench_function("exp out of range", |b| {
        b.iter(|| {
            black_box(exp(-1500));
            black_box(exp(1500));
        })
    });
}

fn bench_play_game(c: &mut Criterion) {
    let mut rng = rand::rng();
    c.bench_function("play_game random", |b| {
        b.iter(|| {
            let s1 = match rng.random_range(0..3) {
                0 => Strategy::Rock,
                1 => Strategy::Paper,
                _ => Strategy::Scissors,
            };
            let s2 = match rng.random_range(0..3) {
                0 => Strategy::Rock,
                1 => Strategy::Paper,
                _ => Strategy::Scissors,
            };
            black_box(play_game(s1, s2));
        })
    });
}

fn bench_play_tournament(c: &mut Criterion) {
    let strategies = random_strategies();
    let scores = random_scores();
    let network = Network::new();
    c.bench_function("play_tournament random", |b| {
        b.iter(|| {
            play_tournament(
                black_box(&strategies),
                black_box(&mut scores.clone()),
                black_box(&network),
            );
        })
    });
}

fn bench_get_local_scores(c: &mut Criterion) {
    let strategies = random_strategies();
    let scores = random_scores();
    let network = Network::new();
    c.bench_function("get_local_scores random", |b| {
        b.iter(|| {
            black_box(get_local_scores(&strategies, &scores, &network));
        })
    });
}

fn bench_get_new_strat(c: &mut Criterion) {
    let mut rng = rand::rng();
    let scores = [
        rng.random_range(0..100),
        rng.random_range(0..100),
        rng.random_range(0..100),
    ];
    c.bench_function("get_new_strat random", |b| {
        b.iter(|| {
            black_box(get_new_strat(black_box(0.5), &scores));
        })
    });
}

fn bench_update_strategies(c: &mut Criterion) {
    let strategies = random_strategies();
    let scores = random_scores();
    let network = Network::new();
    c.bench_function("update_strategies random", |b| {
        b.iter(|| {
            let mut local_strats = strategies.clone();
            update_strategies(
                black_box(&mut local_strats),
                black_box(&scores),
                black_box(&network),
            );
        })
    });
}

criterion_group!(
    new_benchmark,
    bench_exp,
    bench_play_game,
    bench_play_tournament,
    bench_get_local_scores,
    bench_get_new_strat,
    bench_update_strategies
);
criterion_main!(new_benchmark);
