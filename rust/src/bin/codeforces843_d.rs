use std::io;
use std::str;
use std::usize;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::cmp;

static INF: i64 = 1 << 60;
static MAX_REQ: usize = 2000000;

fn main() {
    let start = 0;
    let (n, m, q) = {
        let v = read_values::<usize>();
        (v[0], v[1], v[2])
    };

    let mut graph: Vec<Vec<Edge>> = vec![Vec::new(); n];
    let mut edges = Vec::new();
    for _ in 0..m {
        let (from, to, cost) = {
            let v = read_values::<usize>();
            (v[0] - 1, v[1] - 1, v[2] as i64)
        };
        edges.push((from, graph[from].len()));
        graph[from].push(Edge { to: to, cost: cost });
    }

    let mut shortest_dist = dijkstra(start, &graph);
    let mut deque = vec![VecDeque::new(); MAX_REQ];
    let mut add = vec![INF; n];

    let mut modify_queries = Vec::new();

    for _ in 0..q {
        let queries = read_values::<usize>();
        if queries[0] == 1 {
            let to = queries[1] - 1;
            let num = modify_queries.len();

            for edge_idx in &modify_queries {
                let (from, idx): (usize, usize) = edges[*edge_idx];
                unsafe { graph[from].get_unchecked_mut(idx).cost += 1; }
            }
            modify_queries.clear();

            add[start] = 0;
            deque[0].push_back(start);

            for dist in 0..(num + 1) {
                while !deque[dist].is_empty() {
                    let cur = deque[dist].pop_front().unwrap();
                    if add[cur] != dist as i64 {
                        continue;
                    }
                    for e in &graph[cur] {
                        let d = shortest_dist[cur] + add[cur] - shortest_dist[e.to] + e.cost;
                        if d as usize <= num && add[e.to] > d {
                            add[e.to] = d;
                            deque[d as usize].push_back(e.to);
                        }
                    }
                }
            }

            for i in 0..n {
                shortest_dist[i] = cmp::min(shortest_dist[i] + add[i], INF);
                add[i] = INF;
            }

            if shortest_dist[to] >= INF {
                println!("-1");
            } else {
                println!("{}", shortest_dist[to]);
            }
        } else {
            let num = queries[1];
            for i in 0..num { modify_queries.push(queries[i + 2] - 1); }
        }
    }
}

fn dijkstra(start: usize, graph: &Vec<Vec<Edge>>) -> Vec<i64> {
    let n = graph.len();
    let mut heap = BinaryHeap::<Edge>::new();
    heap.push(Edge { to: start, cost: 0 });
    let mut dist = vec![INF; n];
    dist[start] = 0;
    while !heap.is_empty() {
        let p: Edge = heap.pop().unwrap();
        for e in &graph[p.to] {
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
    cost: i64,
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