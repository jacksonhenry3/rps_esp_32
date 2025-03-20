use lazy_static::lazy_static;
use rand::prelude::*;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

pub const BETA: f32 = 1.0;
pub const NUM_VERTICES: usize = 100 * 100;

lazy_static! {
    static ref EXP_TABLE: [f32; 2001] = {
        let mut table = [0.0; 2001];
        for i in -1000..=1000 {
            table[(i + 1000) as usize] = (i as f32 * BETA).exp();
        }
        table
    };
}

fn exp(n: i32) -> f32 {
    if -1000 <= n && n <= 1000 {
        return EXP_TABLE[(n + 1000) as usize];
    };
    (n as f32 * BETA).exp()
}
//  an adjacency matrix with a constant,runtime set number of vertices
pub struct Network {
    edges: Vec<(usize, usize)>,
    neighbors: Vec<Vec<usize>>,
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

impl Network {
    pub fn new() -> Self {
        let mut neighbors = vec![];
        for i in 0..NUM_VERTICES {
            neighbors.push(vec![i]);
        }
        Network {
            edges: vec![],
            neighbors,
        }
    }

    pub fn add_edge(&mut self, i: usize, j: usize) {
        self.edges.push((i, j));
        self.neighbors[i].push(j);
        self.neighbors[j].push(i);
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Strategy {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

#[derive(Clone, Copy, Debug)]
pub struct Agent {
    pub strategy: Strategy,
    pub score: i32,
}

pub fn play_game(agent1: Agent, agent2: Agent) -> (i32, i32) {
    // create a payoff matrix
    let payoff_matrix = [[1, 0, 2], [2, 1, 0], [0, 2, 1]];
    let agent1_strategy = agent1.strategy as usize;
    let agent2_strategy = agent2.strategy as usize;
    let agent1_payoff = payoff_matrix[agent1_strategy][agent2_strategy];
    let agent2_payoff = payoff_matrix[agent2_strategy][agent1_strategy];
    (agent1_payoff, agent2_payoff)
}

pub fn play_tournament(agents: &mut Vec<Agent>, matrix: &Network) {
    for (i, j) in matrix.edges.iter() {
        let (agent_1_payoff, agent2_payoff) = play_game(agents[*i], agents[*j]);
        agents[*i].score += agent_1_payoff;
        agents[*j].score += agent2_payoff;
    }
}

fn get_new_strat(
    agents: &Vec<Agent>,
    network: &Network,
    index: usize,
    random_number: f32,
) -> Strategy {
    let neighbors = network.neighbors.get(index).unwrap();

    // find the total score for each strategy in the neighborhood
    let mut scores = [0; 3];

    for &j in neighbors {
        scores[agents[j].strategy as usize] += agents[j].score;
    }

    let mut probabilities = [0.0; 3];
    for i in 0..3 {
        probabilities[i] = exp(scores[i]);
    }

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

pub fn update_strategies(agents: &mut Vec<Agent>, matrix: &Network) {
    let mut rng = rand::rng();
    let random_numbers = (0..NUM_VERTICES)
        .map(|_| rng.random::<f32>())
        .collect::<Vec<_>>();

    let new_strats = (0..NUM_VERTICES)
        .into_par_iter()
        .map(|i| get_new_strat(&agents, matrix, i, random_numbers[i]))
        .collect::<Vec<_>>();

    agents
        .par_iter_mut()
        .enumerate()
        .for_each(|(i, agent)| agent.strategy = new_strats[i]);
}

// function to print out the straetgey of every agent in a list. Each such be a verry different looking charachter thats the same width

pub fn print_agents(agents: &[Agent; NUM_VERTICES]) {
    for agent in agents {
        let strategy = match agent.strategy {
            Strategy::Rock => "R",
            Strategy::Paper => "P",
            Strategy::Scissors => "S",
        };
        print!("{:4}", strategy);
    }
    println!();
}

pub fn print_payoffs(agents: &[Agent]) {
    for agent in agents {
        print!("{:4}", agent.score);
    }
    println!();
}
