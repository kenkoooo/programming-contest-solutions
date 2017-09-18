use std::io;
use std::io::{Read, Stdin};
use std::str;
use std::str::FromStr;
use std::usize;
use std::fmt::Debug;
use std::collections::VecDeque;
use std::collections::BTreeSet;

static INVALID_GRUNDY: usize = 200000;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.parse::<usize>();
    let p = (0..n).map(|_| { sc.parse::<usize>() - 1 }).collect::<Vec<_>>();

    let mut graph: Vec<Vec<usize>> = (0..n).map(|_| { Vec::new() }).collect::<Vec<Vec<_>>>();
    for i in 0..n {
        graph[p[i]].push(i);
    }

    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut grundy = vec![INVALID_GRUNDY; n];
    let mut children_count = vec![0; n];

    for i in 0..n {
        if graph[i].is_empty() {
            grundy[i] = 0;
            queue.push_back(i);
        }
    }

    // BFS from leaves
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        let parent = p[v];
        children_count[parent] += 1;

        if children_count[parent] == graph[parent].len() {
            let mut children_set = BTreeSet::new();
            for c in &graph[parent] { children_set.insert(grundy[*c]); }
            for g in 0..n {
                if !children_set.contains(&g) {
                    grundy[parent] = g;
                    break;
                }
            }
            queue.push_back(parent);
        }
    }

    let mut start = n;
    for i in 0..n {
        if grundy[i] == INVALID_GRUNDY {
            start = i;
            break;
        }
    }
    if start == n {
        // if all the grundy numbers are determined, it is clearly possible.
        println!("POSSIBLE");
        return;
    }

    let mut start_children_set = BTreeSet::new();
    for c in &graph[start] {
        if grundy[*c] == INVALID_GRUNDY { continue; }
        start_children_set.insert(grundy[*c]);
    }

    let mut first_candidate = 0;
    loop {
        if !start_children_set.contains(&first_candidate) {
            break;
        }
        first_candidate += 1;
    }
    start_children_set.insert(first_candidate);
    let mut second_candidate = 0;
    loop {
        if !start_children_set.contains(&second_candidate) {
            break;
        }
        second_candidate += 1;
    }
    if cycle_dfs(start, first_candidate, &mut grundy, &graph, &p) {
        println!("POSSIBLE");
    } else if cycle_dfs(start, second_candidate, &mut grundy, &graph, &p) {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}

fn cycle_dfs(k: usize, candidate: usize, grundy: &mut Vec<usize>, graph: &Vec<Vec<usize>>, p: &Vec<usize>) -> bool {
    if grundy[k] != INVALID_GRUNDY {
        if grundy[k] == candidate {
            return true;
        } else {
            return false;
        }
    }

    grundy[k] = candidate;
    let parent = p[k];
    let mut set = BTreeSet::new();
    for c in &graph[parent] {
        set.insert(grundy[*c]);
    }

    let mut parent_candidate = 0;
    loop {
        if !set.contains(&parent_candidate) {
            break;
        }
        parent_candidate += 1;
    }

    if cycle_dfs(parent, parent_candidate, grundy, graph, p) {
        return true;
    } else {
        grundy[k] = INVALID_GRUNDY;
        return false;
    }
}

struct Scanner {
    stdin: Stdin,
    buf: Vec<u8>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            stdin: io::stdin(),
            buf: Vec::with_capacity(256),
        }
    }

    fn parse<T: FromStr>(&mut self) -> T
        where <T as FromStr>::Err: Debug
    {
        self.buf.clear();
        let mut it = self.stdin.lock().bytes();
        let mut c = it.next().unwrap().unwrap();
        while c == ' ' as u8 || c == '\n' as u8 {
            c = it.next().unwrap().unwrap();
        }
        while !(c == ' ' as u8 || c == '\n' as u8) {
            self.buf.push(c);
            c = it.next().unwrap().unwrap();
        }
        str::from_utf8(&self.buf).unwrap().parse::<T>().unwrap()
    }
}