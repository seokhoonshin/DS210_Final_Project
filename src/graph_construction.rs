// graph_construction.rs
use csv::ReaderBuilder;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::NodeIndexable;
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::BufReader};

#[derive(Deserialize)]
struct TeamStats {
    team: String,
    wins: f64,
    losses: f64,
    goals: f64,
    season: String,
}

impl TeamStats {
    // Function to determine the weight of an edge 
    fn calculate_edge_weight(&self, other: &TeamStats) -> i32 {
        // difference in number of wins
        (self.wins - other.wins).abs() as i32
    }
}

pub fn construct_graph(file_path: &str) -> DiGraph<String, i32> {
    let mut graph = DiGraph::<String, i32>::new();
    let mut name_map: HashMap<String, NodeIndex> = HashMap::new();

    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    let mut teams = Vec::new();

    for record in csv_reader.deserialize() {
        let team_stats: TeamStats = match record {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error reading record: {}", e);
                continue;
            }
        };

        teams.push(team_stats);
    }

    // Create nodes for each team
    for team_stats in &teams {
        name_map.entry(team_stats.team.clone())
                .or_insert_with(|| graph.add_node(team_stats.team.clone()));
    }

    // Create edges 
    for team1 in &teams {
        for team2 in &teams {
            if team1.team != team2.team {
                let node1 = *name_map.get(&team1.team).unwrap();
                let node2 = *name_map.get(&team2.team).unwrap();
                let weight = team1.calculate_edge_weight(team2);
                
                graph.add_edge(node1, node2, weight);
            }
        }
    }

    graph
}
