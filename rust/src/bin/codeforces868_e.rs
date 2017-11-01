use std::io;
use std::str;
use std::usize;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::cmp;

fn first_dfs(v: usize, p: usize, tree: &Vec<Vec<Edge>>, criminal: &Vec<usize>)
             -> (i32, usize, usize, usize) {
    //(depth, leaves, crimes, deepest)
    let mut max_depth = 0;
    let mut leaves = 0;
    let mut crimes = criminal[v];
    let mut deepest = 0;
    for e in tree[v].iter() {
        if e.to == p {
            continue;
        }
        let (d, l, c, end) = first_dfs(e.to, v, tree, criminal);

        if max_depth < d + e.weight {
            max_depth = d + e.weight;
            deepest = end;
        }

        crimes += c;
        leaves += l;
    }
    if max_depth == 0 {
        leaves = 1;
        deepest = v;
    }
    (max_depth, leaves, crimes, deepest)
}

fn main() {
    let n = read_values::<usize>()[0];
    let mut tree = vec![Vec::new(); n];

    for _ in 0..(n - 1) {
        let (u, v, w) = {
            let v = read_values::<usize>();
            (v[0] - 1, v[1] - 1, v[2] as i32)
        };
        tree[u].push(Edge { to: v, weight: w });
        tree[v].push(Edge { to: u, weight: w });
    }

    let s = read_values::<usize>()[0] - 1;
    let mut m = read_values::<usize>()[0];

    let criminal = {
        let mut criminal = vec![0; n];
        let v = read_values::<usize>();
        for c in &v {
            criminal[c - 1] += 1;
        }
        criminal
    };

    {
        let p = is_path(s, &tree, &criminal, m);
        if p != 0 {
            println!("{}", p);
            return;
        }
    }

    let total_leaves: usize = {
        let mut leaves = 0;
        for edges in tree.iter() {
            if edges.len() == 1 {
                leaves += 1;
            }
        }
        leaves
    };


    let mut minimum = 1000000000;
    for e in tree[s].iter() {
        let (depth, leaves, crimes, deepest) = first_dfs(e.to, s, &tree, &criminal);
        //                println!("{:?}", (depth + e.weight, leaves, crimes, deepest));

        let mut rest_crimes = m;
        if leaves == 1 {
            rest_crimes -= crimes;
        } else {
            rest_crimes -= 1;
        }

        let mut cur = deepest;
        let mut length = depth + e.weight;
        while rest_crimes > 0 {
            let (d, _, _, next_deepest) = first_dfs(cur, n, &tree, &criminal);
            let mut dist = vec![0; n];
            dfs(cur, n, &tree, &mut dist, 0);

            let mut sub_deepest = cur;
            for i in 0..n {
                if i == next_deepest {
                    continue;
                }

                if dist[sub_deepest] < dist[i] {
                    sub_deepest = i;
                }
            }

            cur = sub_deepest;
            if rest_crimes == 1 {
                length += d;
            } else {
                length += dist[sub_deepest];
            }

            rest_crimes -= 1;
        }
        //        println!("{} {}", e.to, length);
        minimum = cmp::min(minimum, length);
    }

    println!("{}", minimum);
}

fn dfs(v: usize, p: usize, tree: &Vec<Vec<Edge>>, dist: &mut Vec<i32>, d: i32) {
    dist[v] = d;
    for e in tree[v].iter() {
        if e.to == p {
            continue;
        }
        dfs(e.to, v, tree, dist, d + e.weight);
    }
}


#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    to: usize,
    weight: i32,
}


fn is_path(s: usize, tree: &Vec<Vec<Edge>>, criminal: &Vec<usize>, m: usize) -> i32 {
    let mut leaves = 0;
    for edges in tree.iter() {
        if edges.len() == 1 {
            leaves += 1;
        }
    }

    if leaves != 2 {
        0
    } else if tree[s].len() == 1 {
        let (depth, hunt) = path_dfs(s, tree.len(), tree, criminal);
        depth
    } else {
        let to1 = tree[s][0].to;
        let to2 = tree[s][1].to;
        let (depth1, hunt1) = path_dfs(s, to2, tree, criminal);
        let (depth2, hunt2) = path_dfs(s, to1, tree, criminal);

        if hunt1 == 0 {
            depth2
        } else if hunt2 == 0 {
            depth1
        } else {
            cmp::min(depth1, depth2) + depth1 + depth2
        }
    }
}

fn path_dfs(v: usize, p: usize, tree: &Vec<Vec<Edge>>, criminal: &Vec<usize>) -> (i32, usize) {
    let mut depth = 0;
    let mut hunt = criminal[v];
    for e in tree[v].iter() {
        if e.to == p {
            continue;
        }

        let (t_depth, t_hunt) = path_dfs(e.to, v, tree, criminal);
        depth = t_depth + e.weight;
        hunt += t_hunt;
    }
    (depth, hunt)
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