use std::io;
use std::io::{Read, Stdin};
use std::str;
use std::str::FromStr;
use std::usize;
use std::fmt::Debug;
use std::collections::VecDeque;

struct State {
    s: String,
    v: usize,
}

fn main() {
    let mut sc = Scanner::new();
    let a = sc.parse::<String>().into_bytes();
    let n = a.len();
    let mut pos = (0..26).map(|_| {
        VecDeque::new()
    }).collect::<Vec<_>>();
    for i in 0..n {
        let c = (a[i] - 'a' as u8) as usize;
        pos[c].push_back(i);
    }

    let mut graph = (0..(n + 2)).map(|_| { Vec::new() }).collect::<Vec<_>>();
    let source = n;
    let sink = n + 1;

    for next in 0..26 {
        if pos[next].is_empty() {
            graph[source].push(sink);
        } else {
            let x = pos[next].front().unwrap();
            graph[source].push(*x);
        }
    }

    for i in 0..n {
        let c = (a[i] - 'a' as u8) as usize;
        pos[c].pop_front();
        for next in 0..26 {
            if pos[next].is_empty() {
                graph[i].push(sink);
            } else {
                let x = pos[next].front().unwrap();
                graph[i].push(*x);
            }
        }
    }

    let mut q = VecDeque::new();
    let mut dist = vec![n * 2; n + 2];
    q.push_back(State { s: "".to_owned(), v: source });
    dist[source] = 0;
    while !q.is_empty() {
        let state = q.pop_front().unwrap();
        if state.v == sink {
            println!("{}", state.s);
            return;
        }

        for i in 0..26 {
            let c = (i as u8 + 'a' as u8) as char;
            let next = graph[state.v][i];
            if dist[next] <= dist[state.v] + 1 {
                continue;
            }
            dist[next] = dist[state.v] + 1;
            q.push_back(State { s: state.s.clone() + &c.to_string(), v: next });
        }
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