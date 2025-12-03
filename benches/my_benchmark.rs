// use criterion::{Criterion, criterion_group, criterion_main};
// use rand::random;
// use rps::*;
// use std::hint::black_box;

// fn play_benchmark(c: &mut Criterion) {
//     let mut strategies = vec![Strategy::Rock; NUM_VERTICES];
//     let mut scores = vec![1; NUM_VERTICES];
//     for index in 1..NUM_VERTICES {
//         let random_strategy = match rand::random::<u32>() % 3 + 1 {
//             1 => Strategy::Rock,
//             2 => Strategy::Paper,
//             3 => Strategy::Scissors,
//             _ => panic!("Invalid random number"),
//         };
//         strategies[index] = random_strategy;
//     }
//     let matrix = Network::new();

//     c.bench_function("Play benchmark", |b| {
//         b.iter(|| {
//             play_tournament(
//                 black_box(&strategies),
//                 black_box(&mut scores),
//                 black_box(&matrix),
//             );
//         })
//     });
// }

// fn update_benchmark(c: &mut Criterion) {
//     let mut strategies = vec![Strategy::Rock; NUM_VERTICES];
//     let mut scores = vec![1; NUM_VERTICES];
//     for index in 1..NUM_VERTICES {
//         let random_strategy = match rand::random::<u32>() % 3 + 1 {
//             1 => Strategy::Rock,
//             2 => Strategy::Paper,
//             3 => Strategy::Scissors,
//             _ => panic!("Invalid random number"),
//         };
//         strategies[index] = random_strategy;
//     }

//     let matrix = Network::new();

//     c.bench_function("Update benchmark", |b| {
//         b.iter(|| {
//             update_strategies(
//                 black_box(&mut strategies),
//                 black_box(&scores),
//                 black_box(&matrix),
//             );
//         })
//     });
// }

// criterion_group!(benches, play_benchmark, update_benchmark,);
// criterion_main!(benches);
