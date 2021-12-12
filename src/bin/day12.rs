use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    iter,
};

fn is_lowercase(s: &str) -> bool {
    s.to_lowercase() == s
}

#[derive(Debug)]
struct Path {
    path: Vec<String>,
    neighbors: Vec<String>,
}

impl Path {
    fn start(graph: &Graph) -> Path {
        Path {
            path: vec!["start".to_string()],
            neighbors: graph.get("start").unwrap().clone(),
        }
    }

    fn is_finished(&self) -> bool {
        self.path.last().unwrap() == "end"
    }

    fn has_lowercase_duplicates(&self) -> bool {
        self.path
            .iter()
            .filter(|&node| is_lowercase(node))
            .cloned()
            .collect::<HashSet<String>>()
            .len()
            < self.path.iter().filter(|&node| is_lowercase(node)).count()
    }

    fn part1_filter(&self, node: &str) -> bool {
        !(node.to_lowercase() == node && self.path.contains(&node.to_string()))
    }

    fn part2_filter(&self, node: &str) -> bool {
        !(node == "start"
            || (is_lowercase(node)
                && self.path.contains(&node.to_string())
                && self.has_lowercase_duplicates()))
    }

    fn go_to_node(&self, graph: &Graph, node: &str) -> Path {
        let mut path = Path {
            path: self
                .path
                .iter()
                .cloned()
                .chain(iter::once(node.to_string()))
                .collect(),
            neighbors: Vec::new(),
        };

        if node != "end" {
            path.neighbors = graph
                .get(node)
                .unwrap()
                .iter()
                .cloned()
                .filter(|node| path.part2_filter(node))
                .collect()
        }

        path
    }

    fn neighbors_paths(&self, graph: &Graph) -> Vec<Path> {
        let mut paths = Vec::new();

        for node in self.neighbors.iter() {
            paths.push(self.go_to_node(graph, node));
        }

        paths
    }

    fn all_paths(graph: &Graph) -> Vec<Path> {
        let mut paths = vec![Path::start(graph)];
        let mut completed_paths = Vec::new();

        while let Some(path) = paths.pop() {
            for path in path.neighbors_paths(graph) {
                paths.push(path);
            }

            if path.is_finished() {
                completed_paths.push(path);
            }
        }

        completed_paths
    }
}

type Graph = HashMap<String, Vec<String>>;

fn build_graph(data: &str) -> Graph {
    let data = data.lines().map(|line| line.split_once('-').unwrap());
    let mut graph: Graph = HashMap::new();

    for (a, b) in data {
        graph
            .entry(a.to_string())
            .or_insert_with(Vec::new)
            .push(b.to_string());
        graph
            .entry(b.to_string())
            .or_insert_with(Vec::new)
            .push(a.to_string())
    }

    graph
}

fn main() {
    let input = read_to_string("input/day12/input").unwrap();
    let graph = build_graph(&input);

    let all_paths = Path::all_paths(&graph);
    println!("{}", all_paths.len());
}
