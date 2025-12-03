#![no_std]

use rand::prelude::*;
use core::array;

pub const BETA: f32 = 1.0;
pub const NUM_VERTICES: usize = 64 * 64;
pub const PAYOFF_MATRIX: [[i32; 3]; 3] = [[1, 0, 2], [2, 1, 0], [0, 2, 1]];

include!(concat!(env!("OUT_DIR"), "/exp_table.rs"));

pub fn get_val(i: usize) -> f32 {
    TABLE[i]
}



pub fn exp(n: i32) -> f32 {
    if -1000 <= n && n <= 1000 {
        return get_val((n + 1000) as usize);
    };
    (n as f32 * BETA).exp()
}


pub struct Network {
    edges: [(usize, usize);NUM_VERTICES],
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

impl Network {
    pub fn new() -> Self {
        let mut edges = [(0,0);NUM_VERTICES];
        for i in 0..NUM_VERTICES {
            edges[i] =(i, (i + 1) % NUM_VERTICES);
        }
        Network { edges }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Strategy {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

pub fn play_game(agent_1_strategy: Strategy, agent_2_strategy: Strategy) -> (i32, i32) {
    // create a payoff matrix
    let agent1_strategy = agent_1_strategy as usize;
    let agent2_strategy = agent_2_strategy as usize;
    let agent1_payoff = PAYOFF_MATRIX[agent1_strategy][agent2_strategy];
    let agent2_payoff = PAYOFF_MATRIX[agent2_strategy][agent1_strategy];
    (agent1_payoff, agent2_payoff)
}

pub fn play_tournament(strategies: &[Strategy], scores: &mut [i32], network: &Network) {
    for (agent_1_id, agent_2_id) in network.edges.iter() {
        let (agent_1_payoff, agent2_payoff) =
            play_game(strategies[*agent_1_id], strategies[*agent_2_id]);
        scores[*agent_1_id] += agent_1_payoff;
        scores[*agent_2_id] += agent2_payoff;
    }
}

pub fn get_local_scores(
    strategies: &[Strategy],
    scores: &[i32],
    network: &Network,
) -> [[i32; 3];NUM_VERTICES] {
    // returns an array whos elements are the local coinstructed sciore of each strategy  as
    // measured by the onode at that index.
    let mut results = [[0, 0, 0]; NUM_VERTICES];
    for (agent1_id, agent2_id) in network.edges.iter() {
        let (agent_1_strategy, agent_2_strategy) = (strategies[*agent1_id], strategies[*agent2_id]);

        results[*agent2_id][agent_1_strategy as usize] += scores[*agent1_id];
        results[*agent1_id][agent_2_strategy as usize] += scores[*agent2_id];
    }

    for agent_id in 0..NUM_VERTICES {
        results[agent_id][strategies[agent_id] as usize] += scores[agent_id];
    }

    results
}

pub fn get_new_strat(random_number: f32, scores: &[i32; 3]) -> Strategy {
    let probabilities = [exp(scores[0]), exp(scores[1]), exp(scores[2])];

    // Calculate total without cloning
    let total: f32 = probabilities.iter().sum();
    let threshold = random_number * total;

    // Use early returns to avoid unnecessary iterations
    let mut cumulative = probabilities[0];
    if threshold < cumulative {
        return Strategy::Rock;
    }

    cumulative += probabilities[1];
    if threshold < cumulative {
        return Strategy::Paper;
    }

    // No need to check the last condition
    Strategy::Scissors
}

pub fn update_strategies(strategies: &mut [Strategy; NUM_VERTICES], scores: &[i32], network: &Network) {
    let mut rng = rand::rng();
    // let random_numbers:[f32; NUM_VERTICES] = (0..NUM_VERTICES)
    //     .map(|_| rng.random::<f32>())
    //     .collect();

    let local_scores = get_local_scores(strategies, scores, network);
    // let new_strats = (0..NUM_VERTICES)
    //     // .into_par_iter()
    //     .map(|i| get_new_strat(random_numbers[i], &local_scores[i]))
    //     .collect();

    let new_strats= array::from_fn(|i| get_new_strat(rng.random::<f32>(), &local_scores[i]));
    *strategies = new_strats;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a custom test Network.
    fn test_network(edges_vec: &[(usize, usize)]) -> Network {
        // Fill with a harmless edge to a vertex with zero score so extra edges don't affect tests.
        let filler = (NUM_VERTICES - 1, NUM_VERTICES - 1);
        let mut edges = [filler; NUM_VERTICES];
        for (i, &e) in edges_vec.iter().enumerate() {
            edges[i] = e;
        }
        Network { edges }
    }

    #[test]
    fn test_play_game_mechanics() {
        // Test a known matchup: Rock vs. Paper.
        let (score1, score2) = play_game(Strategy::Rock, Strategy::Paper);
        let expected1 = PAYOFF_MATRIX[Strategy::Rock as usize][Strategy::Paper as usize];
        let expected2 = PAYOFF_MATRIX[Strategy::Paper as usize][Strategy::Rock as usize];
        assert_eq!(score1, expected1);
        assert_eq!(score2, expected2);
    }

    #[test]
    fn test_get_local_scores_mechanics() {
        // Construct a small network with 3 meaningful edges at the beginning.
        // Edges: (0,1), (1,2), (2,0)
        let network = test_network(&[(0, 1), (1, 2), (2, 0)]);

        // Create full-size arrays but only set the first three vertices to meaningful values.
        let mut strategies = [Strategy::Rock; NUM_VERTICES];
        strategies[0] = Strategy::Rock;
        strategies[1] = Strategy::Paper;
        strategies[2] = Strategy::Scissors;

        let mut scores = [0_i32; NUM_VERTICES];
        scores[0] = 10;
        scores[1] = 20;
        scores[2] = 30;

        // Expected local scores for vertices 0..2; rest remain zeros.
        let mut expected = [[0_i32; 3]; NUM_VERTICES];
        expected[0] = [10, 20, 30];
        expected[1] = [10, 20, 30];
        expected[2] = [10, 20, 30];

        let computed = get_local_scores(&strategies, &scores, &network);
        assert_eq!(computed, expected);
    }

    #[test]
    fn test_get_new_strat_mechanics() {
        // With equal scores (0,0,0) probabilities are [1,1,1].
        // Total = 3.
        // For random number 0.2, threshold=0.6 (<1) → Rock.
        assert_eq!(get_new_strat(0.2, &[0, 0, 0]), Strategy::Rock);
        // For random number 0.4, threshold=1.2 (falls in second slot) → Paper.
        assert_eq!(get_new_strat(0.4, &[0, 0, 0]), Strategy::Paper);
        // For random number 0.9, threshold=2.7 (third slot) → Scissors.
        assert_eq!(get_new_strat(0.9, &[0, 0, 0]), Strategy::Scissors);
    }

    #[test]
    fn test_play_tournament_mechanics() {
        // Create a 2-vertex network with two directed edges placed at the beginning.
        let network = test_network(&[(0, 1), (1, 0)]);

        // Full-size arrays with only indices 0 and 1 meaningful.
        let mut strategies = [Strategy::Rock; NUM_VERTICES];
        strategies[0] = Strategy::Rock;
        strategies[1] = Strategy::Paper;

        let mut scores = [0_i32; NUM_VERTICES];
        play_tournament(&strategies, &mut scores, &network);

        // For edge (0,1) and (1,0): each matchup occurs twice.
        let expected_score0 = PAYOFF_MATRIX[Strategy::Rock as usize][Strategy::Paper as usize] * 2;
        let expected_score1 = PAYOFF_MATRIX[Strategy::Paper as usize][Strategy::Rock as usize] * 2;
        assert_eq!(scores[0], expected_score0);
        assert_eq!(scores[1], expected_score1);
    }

    #[test]
    fn test_update_strategies_statistical() {
        let iterations = 1000;
        let mut counts = [0; 3]; // counts for Rock, Paper, Scissors

        // Network with a single meaningful edge at index 0 (self-edge for vertex 0).
        let network = test_network(&[(0, 0)]);

        // Scores array with zeros -> uniform probabilities.
        let scores = [0_i32; NUM_VERTICES];

        for _ in 0..iterations {
            // Full-size strategies array; we inspect index 0 only.
            let mut strategies = [Strategy::Rock; NUM_VERTICES];
            update_strategies(&mut strategies, &scores, &network);
            match strategies[0] {
                Strategy::Rock => counts[0] += 1,
                Strategy::Paper => counts[1] += 1,
                Strategy::Scissors => counts[2] += 1,
            }
        }
        // With uniform probabilities, each strategy should be chosen ~iterations/3 times.
        let expected = iterations as f32 / 3.0;
        let tolerance = iterations as f32 * 0.1; // 10% tolerance.
        for (i, &count) in counts.iter().enumerate() {
            assert!(
                (count as f32 - expected).abs() <= tolerance,
                "Strategy {} occurred {} times; expected approx {} (±{})",
                i,
                count,
                expected,
                tolerance
            );
        }
    }
}

