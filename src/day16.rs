use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

use ndarray::Array2;
use priority_queue::PriorityQueue;
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

fn potential_flow(flows: &[u32], open_valves: &[bool], time: u32) -> u32 {
    flows
        .iter()
        .zip(open_valves.iter())
        .filter(|(_, &x)| !x)
        .map(|(x, _)| x)
        .sum::<u32>()
        * time
}

fn solve_dijkstra(
    flows: &[u32],
    distmat: &Array2<u32>,
    open_valves: Vec<bool>,
    time: u32,
) {
    let mut queue = PriorityQueue::new();

    let potential = potential_flow(flows, &open_valves, time);

    queue.push((0, open_valves, time), potential);

    while let Some(((pos, open_valves, time_left), potential)) = queue.pop() {
        for npos in (0..open_valves.len()).filter(|&i| !open_valves[i]) {
            let d = distmat[[pos, npos]];

            if time_left >= d + 1 {
                let mut new_open_valves = open_valves.clone();
                new_open_valves[npos] = true;

                let time_after_move = time_left - d - 1;

                let potential_after_move =
                    potential_flow(flows, &new_open_valves, time_after_move);

                let k = (npos, new_open_valves, time_after_move);

                if let Some(old_pot) = queue.get_priority(&k) {
                    // update prio
                }
            }
        }
    }
}

fn part1() {
    let (flows, distmat) = parse_input("input/day16/ex1");

    solve_dijkstra(&flows, &distmat, vec![false; flows.len()], 30);
}

fn part2() {}
