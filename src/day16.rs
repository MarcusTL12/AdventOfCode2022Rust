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

fn instantaneous_potential_flow(flows: &[u32], mut open_valves: u32) -> u32 {
    flows
        .iter()
        .filter(|_| {
            let b = open_valves & 1 == 0;
            open_valves >>= 1;
            b
        })
        .sum::<u32>()
}

fn solve_dijkstra(
    queue: &mut PriorityQueue<(u32, u32, u32), u32>,
    flows: &[u32],
    distmat: &Array2<u32>,
    open_valves: u32,
    time: u32,
    lower_bound: u32,
) -> u32 {
    queue.clear();

    let max_potential = instantaneous_potential_flow(flows, open_valves) * time;

    queue.push((0, open_valves, time), max_potential);

    while let Some(((pos, open_valves, time_left), potential)) = queue.pop() {
        if time_left == 0 {
            return potential;
        }

        let mut added_nodes = false;

        let mut open_valves_cpy = open_valves;
        for npos in (0..flows.len()).filter(|_| {
            let b = open_valves_cpy & 1 == 0;
            open_valves_cpy >>= 1;
            b
        }) {
            let d = distmat[[pos as usize, npos]] + 1;

            if time_left >= d {
                added_nodes = true;

                let new_open_valves = open_valves | (1 << npos);

                let time_after_move = time_left - d;

                let potential_lost =
                    instantaneous_potential_flow(flows, open_valves) * d;

                let k = (npos as u32, new_open_valves, time_after_move);

                let new_potential = potential - potential_lost;

                if new_potential >= lower_bound {
                    queue.push_increase(k, new_potential);
                }
            }
        }

        if !added_nodes {
            let potential_lost =
                instantaneous_potential_flow(flows, open_valves) * time_left;

            let new_potential = potential - potential_lost;

            if new_potential >= lower_bound {
                queue.push_increase((pos, open_valves, 0), new_potential);
            }
        }
    }

    0
}

fn part1() {
    let (flows, distmat) = parse_input("input/day16/input");

    let ans = solve_dijkstra(
        &mut PriorityQueue::new(),
        &flows,
        &distmat,
        1,
        30,
        1500,
    );

    println!("{ans}");
}

fn part2() {
    let (flows, distmat) = parse_input("input/day16/input");

    let lower_bound = 1250;

    let ans = (0..2_u32.pow(flows.len() as u32 - 1))
        .into_par_iter()
        .map_init(
            || PriorityQueue::new(),
            |queue, i| {
                let my_valves = (i << 1) | 1;
                let el_valves = ((!i) << 1) | 1;

                solve_dijkstra(
                    queue,
                    &flows,
                    &distmat,
                    my_valves,
                    26,
                    lower_bound,
                ) + solve_dijkstra(
                    queue,
                    &flows,
                    &distmat,
                    el_valves,
                    26,
                    lower_bound,
                )
            },
        )
        .max()
        .unwrap();

    println!("{ans}");
}
