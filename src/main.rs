use rps::*;

fn main() {
    let mut strategies = vec![Strategy::Rock; NUM_VERTICES];
    let mut scores = vec![1; NUM_VERTICES];
    for index in 1..NUM_VERTICES {
        let random_strategy = match rand::random::<u32>() % 3 + 1 {
            1 => Strategy::Rock,
            2 => Strategy::Paper,
            3 => Strategy::Scissors,
            _ => panic!("Invalid random number"),
        };
        strategies[index] = random_strategy;
    }

    let matrix = Network::new();

    // do 100 iterations of the tournament
    for i in 0..100 {
        println!("{}", i);
        play_tournament(&strategies, &mut scores, &matrix);
        update_strategies(&mut strategies, &scores, &matrix);
        // print_agents(&agents);
    }
}
