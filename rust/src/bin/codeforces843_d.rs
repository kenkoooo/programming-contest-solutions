use std::io;
use std::str;
use std::usize;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::cmp;
use std::usize::MAX;
use std::io::Write;

static INF: u64 = 1 << 60;
static Q_MAX: usize = 1000000;

fn main() {
    let start = 0;

    let (n, m, q) = {
        let v = read_values::<usize>();
        (v[0], v[1], v[2])
    };
    let mut graph = vec![Vec::new(); n];
    let mut edge_indices = Vec::new();
    for _ in 0..m {
        let (from, to, cost) = {
            let v = read_values::<usize>();
            (v[0] - 1, v[1] - 1, v[2] as u64)
        };
        edge_indices.push((from, graph[from].len()));
        graph[from].push(Edge { to: to, cost: cost });
    }

    let mut shortest_dist = dijkstra(start, &graph);
    let mut deque = vec![VecDeque::new(); Q_MAX];
    let mut add = vec![INF; n];
    let mut modified_count = 0;
    for _ in 0..q {
        let input = read_values::<usize>();
        if input[0] == 2 {
            let count = input[1];
            for i in 0..count {
                let (from, idx) = edge_indices[input[i + 2] - 1];
                unsafe { graph[from].get_unchecked_mut(idx).cost += 1; }
                modified_count += 1;
            }
            continue;
        }

        let dest = input[1] - 1;
        add[start] = 0;
        deque[0].push_back(start);
        for dist in 0..(modified_count + 1) {
            while !deque[dist].is_empty() {
                let v = deque[dist].pop_front().unwrap();
                if add[v] != dist as u64 { continue; }
                for edge in graph[v].iter() {
                    let d = add[v] + shortest_dist[v] + edge.cost - shortest_dist[edge.to];
                    if d <= modified_count as u64 && add[edge.to] > d {
                        add[edge.to] = d;
                        deque[d as usize].push_back(edge.to);
                    }
                }
            }
        }

        for i in 0..n {
            shortest_dist[i] = cmp::min(shortest_dist[i] + add[i], INF);
            add[i] = INF;
        }

        if shortest_dist[dest] >= INF {
            io::stdout().write(b"-1");
        } else {
            //            println!("{}", shortest_dist[dest]);
            io::stdout().write(shortest_dist[dest].to_string().as_bytes());
        }
        io::stdout().write(b"\n");

        modified_count = 0;
    }
}

fn dijkstra(from: usize, graph: &Vec<Vec<Edge>>) -> Vec<u64> {
    let n = graph.len();
    let mut heap = BinaryHeap::new();
    heap.push(Edge { to: from, cost: 0 });
    let mut dist = vec![INF; n];
    dist[0] = 0;
    while !heap.is_empty() {
        let p = heap.pop().unwrap();
        for e in graph[p.to].iter() {
            if dist[e.to] > e.cost + dist[p.to] {
                dist[e.to] = e.cost + dist[p.to];
                heap.push(Edge { to: e.to, cost: dist[e.to] });
            }
        }
    }
    dist
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    to: usize,
    cost: u64,
}

impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read_values<T>() -> Vec<T>
    where T: std::str::FromStr,
          T::Err: std::fmt::Debug
{
    read_line()
        .split(' ')
        .map(|a| a.trim().parse().unwrap())
        .collect()
}