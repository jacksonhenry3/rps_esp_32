// load rand
extern crate rand;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

//  an adjacency matrix with a constant,runtime set number of vertices
pub const NUM_VERTICES: usize = 100;
pub struct AdjacencyMatrix {
    matrix: [[bool; NUM_VERTICES]; NUM_VERTICES],
}

impl AdjacencyMatrix {
    pub fn new() -> Self {
        AdjacencyMatrix {
            matrix: [[false; NUM_VERTICES]; NUM_VERTICES],
        }
    }

    pub fn add_edge(&mut self, i: usize, j: usize) {
        self.matrix[i][j] = true;
        self.matrix[j][i] = true;
    }

    pub fn remove_edge(&mut self, i: usize, j: usize) {
        self.matrix[i][j] = false;
        self.matrix[j][i] = false;
    }

    pub fn has_edge(&self, i: usize, j: usize) -> bool {
        self.matrix[i][j]
    }

    pub fn neighbors(&self, i: usize) -> Vec<usize> {
        // return the indices of "true"s in the row
        let mut neighbors = Vec::new();
        for j in 0..NUM_VERTICES {
            if self.matrix[i][j] {
                neighbors.push(j);
            }
        }
        neighbors
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

pub fn play_tournament(matrix: &AdjacencyMatrix, agents: Vec<Agent>) -> Vec<Agent> {
    let mut new_agents = agents.clone();
    for i in 0..NUM_VERTICES {
        let neighbors = matrix.neighbors(i);
        for j in neighbors {
            let (agent1_payoff, agent2_payoff) = play_game(agents[i], agents[j]);
            new_agents[i].score += agent1_payoff;
            new_agents[j].score += agent2_payoff;
        }
    }
    new_agents
}

pub fn update_strategies(agents: Vec<Agent>, matrix: &AdjacencyMatrix) -> Vec<Agent> {
    let beta = 1.;
    let mut new_agents = agents.clone();
    for i in 0..NUM_VERTICES {
        let neighbors = matrix.neighbors(i);

        // find the total score for each strategy in the neighborhood
        let mut scores = [0; 3];
        for j in neighbors {
            scores[agents[j].strategy as usize] += agents[j].score;
        }

        // randomly select a strategy according to a modified boltzmann distribution
        let mut total = 0.;
        for score in &scores {
            // sum the exponentials
            total += (*score as f32 * beta).exp();
        }

        let probabilities = scores
            .iter()
            .map(|score| (*score as f32 * beta).exp() / total)
            .collect::<Vec<f32>>();

        // randomly select a strategy weighted by the probabilities
        let mut cumulative = 0.;
        let random_number = rand::random::<f32>();
        let mut new_strategy = Strategy::Rock;

        for (strategy, &probability) in [Strategy::Rock, Strategy::Paper, Strategy::Scissors]
            .iter()
            .zip(probabilities.iter())
        {
            cumulative += probability;
            if random_number < cumulative {
                new_strategy = *strategy;
                break;
            }
        }

        new_agents[i].strategy = new_strategy;
    }
    new_agents
}

// function to print out the straetgey of every agent in a list. Each such be a verry different looking charachter thats the same width

pub fn print_agents(agents: &Vec<Agent>) {
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

pub fn print_payoffs(agents: &Vec<Agent>) {
    for agent in agents {
        print!("{:4}", agent.score);
    }
    println!();
}

fn main() {
    let mut matrix = AdjacencyMatrix::new();
    // create a m,atrix for a loop of 10 agents
    for i in 0..NUM_VERTICES {
        matrix.add_edge(i, (i + 1) % NUM_VERTICES);
    }
    // create a vector of agents with random strategies
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

    // do 100 iterations of the tournament
    for _ in 0..100 {
        agents = play_tournament(&matrix, agents);
        agents = update_strategies(agents, &matrix);
        print_agents(&agents);
    }
}
