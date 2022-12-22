use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

use ndarray::Array2;
use priority_queue::PriorityQueue;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn parse_input(filename: &str) -> (Vec<u32>, Array2<u32>) {
    let reg = Regex::new(
        r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)",
    )
    .unwrap();

    let mut name_translation = HashMap::new();
    name_translation.insert("AA".to_owned(), 0);

    let mut translate = |name: &str| {
        if let Some(&id) = name_translation.get(name) {
            id
        } else {
            let id = name_translation.len();
            name_translation.insert(name.to_owned(), id);
            id
        }
    };

    let mut nonzero_translation = HashMap::new();

    nonzero_translation.insert(0, 0); // AA

    let mut full_graph: HashMap<_, Vec<_>> = HashMap::new();

    let mut flows = Vec::new();

    let inp = read_to_string(filename).unwrap();

    for c in inp.split('\n').filter_map(|l| reg.captures(l)) {
        let name = &c[1];
        let flow: u32 = c[2].parse().unwrap();

        let id = translate(name);

        if let Some(x) = flows.get_mut(id) {
            *x = flow;
        } else {
            flows.resize(id + 1, flow);
        }

        if flow != 0 {
            nonzero_translation.insert(id, nonzero_translation.len());
        }

        full_graph.insert(id, c[3].split(", ").map(&mut translate).collect());
    }

    let n = nonzero_translation.len();

    let mut nonzero_flows = vec![0; n];

    let mut distmat = Array2::from_elem([n, n], 0);

    let mut bfs_queue = VecDeque::new();
    let mut bfs_visit = vec![false; full_graph.len()];

    for (&startpoint, &i) in &nonzero_translation {
        nonzero_flows[i] = flows[startpoint];

        bfs_queue.clear();
        bfs_visit.fill(false);

        bfs_queue.push_back((startpoint, 0));
        bfs_visit[startpoint] = true;

        while let Some((node, l)) = bfs_queue.pop_front() {
            if flows[node] != 0 {
                distmat[[i, nonzero_translation[&node]]] = l;
            }

            for &tunnel in &full_graph[&node] {
                if !bfs_visit[tunnel] {
                    bfs_queue.push_back((tunnel, l + 1));
                    bfs_visit[tunnel] = true;
                }
            }
        }
    }

    (nonzero_flows, distmat)
}

fn instantaneous_potential_flow(flows: &[u32], open_valves: &[bool]) -> u32 {
    flows
        .iter()
        .zip(open_valves.iter())
        .filter(|(_, &x)| !x)
        .map(|(x, _)| x)
        .sum::<u32>()
}

fn solve_dijkstra(
    flows: &[u32],
    distmat: &Array2<u32>,
    open_valves: Vec<bool>,
    time: u32,
) -> u32 {
    let mut queue = PriorityQueue::new();

    let max_potential =
        instantaneous_potential_flow(flows, &open_valves) * time;

    queue.push((0, open_valves, time), max_potential);

    while let Some(((pos, open_valves, time_left), potential)) = queue.pop() {
        if time_left == 0 {
            return potential;
        }

        let mut added_nodes = false;

        for npos in (0..open_valves.len()).filter(|&i| !open_valves[i]) {
            let d = distmat[[pos, npos]] + 1;

            if time_left >= d {
                added_nodes = true;

                let mut new_open_valves = open_valves.clone();
                new_open_valves[npos] = true;

                let time_after_move = time_left - d;

                let potential_lost =
                    instantaneous_potential_flow(flows, &open_valves) * d;

                let k = (npos, new_open_valves, time_after_move);

                queue.push_increase(k, potential - potential_lost);
            }
        }

        if !added_nodes {
            let potential_lost =
                instantaneous_potential_flow(flows, &open_valves) * time_left;

            queue.push_increase(
                (pos, open_valves, 0),
                potential - potential_lost,
            );
        }
    }

    panic!("Did not find solution");
}

fn part1() {
    let (flows, distmat) = parse_input("input/day16/input");

    let mut open_valves = vec![false; flows.len()];
    open_valves[0] = true;

    let ans = solve_dijkstra(&flows, &distmat, open_valves, 30);

    println!("{ans}");
}

fn part2() {
    let (flows, distmat) = parse_input("input/day16/input");

    let ans = (0..2_u32.pow(flows.len() as u32 - 1))
        .into_par_iter()
        .map(|i| {
            let mut my_valves = vec![false; flows.len()];
            let mut el_valves = vec![false; flows.len()];

            my_valves[0] = true;
            el_valves[0] = true;

            let mut k = i;

            for j in 1..flows.len() {
                let b = k & 1 == 0;
                my_valves[j] = b;
                el_valves[j] = !b;
                k >>= 1;
            }

            solve_dijkstra(&flows, &distmat, my_valves, 26)
                + solve_dijkstra(&flows, &distmat, el_valves, 26)
        })
        .max()
        .unwrap();

    println!("{ans}");
}
