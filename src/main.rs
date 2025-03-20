use rps::*;

fn main() {
    let mut matrix = Network::new();
    // create a m,atrix for a loop of 10 agents
    for i in 0..NUM_VERTICES {
        matrix.add_edge(i, (i + 1) % NUM_VERTICES);
    }
    // create a vector of agents with random strategies
    let mut agents = vec![
        Agent {
            strategy: Strategy::Rock,
            score: 1,
        };
        NUM_VERTICES
    ];
    for i in 0..NUM_VERTICES {
        let random_strategy = match rand::random::<u32>() % 3 {
            0 => Strategy::Rock,
            1 => Strategy::Paper,
            2 => Strategy::Scissors,
            _ => panic!("Invalid random number"),
        };
        agents[i].strategy = random_strategy;
    }

    // do 100 iterations of the tournament
    for i in 0..100 {
        println!("{}", i);
        play_tournament(&mut agents, &matrix);
        update_strategies(&mut agents, &matrix);
        // print_agents(&agents);
    }
}
