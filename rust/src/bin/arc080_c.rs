use std::io;
use std::io::{Read, Stdin};
use std::str;
use std::str::FromStr;
use std::usize;
use std::cmp;
use std::fmt::Debug;
use std::i64::MAX;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Candidate {
    minimum: i64,
    from: usize,
    until: usize,
}

impl Ord for Candidate {
    fn cmp(&self, other: &Candidate) -> Ordering {
        other.minimum.cmp(&self.minimum)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Candidate) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n = sc.parse::<usize>();
    let arr = (0..n).map(|_| { sc.parse::<i64>() - 1 }).collect::<Vec<_>>();
    let mut rev = vec![0; n];
    for i in 0..n {
        rev[arr[i] as usize] = i;
    }

    let mut even_rmq = RangeMinimumQuery::new(n);
    let mut odd_rmq = RangeMinimumQuery::new(n);
    for i in 0..n {
        if i % 2 == 0 {
            even_rmq.update(i, arr[i]);
        } else {
            odd_rmq.update(i, arr[i]);
        }
    }

    let mut ans = Vec::new();
    let mut heap = BinaryHeap::new();
    heap.push(Candidate { minimum: 0, from: 0, until: n });
    while !heap.is_empty() {
        let s: Candidate = heap.pop().unwrap();
        let from = s.from;
        let until = s.until;

        let (head, tail) = {
            let (ref rmq1, ref rmq2) = if from % 2 == 0 { (&even_rmq, &odd_rmq) } else { (&odd_rmq, &even_rmq) };
            let minimum_value = rmq1.query(from, until);
            let minimum_index = rev[minimum_value as usize];
            let pair = rmq2.query(minimum_index + 1, until);
            let pair_index = rev[pair as usize];
            (minimum_index, pair_index)
        };
        ans.push(arr[head]);
        ans.push(arr[tail]);

        for &(left, right) in &[(from, head), (head + 1, tail), (tail + 1, until)] {
            if left >= right { continue; }
            let minimum = if left % 2 == 0 { even_rmq.query(left, right) } else { odd_rmq.query(left, right) };
            heap.push(Candidate { minimum: minimum, from: left, until: right });
        }
    }

    for t in &ans { print!("{} ", t + 1); }
    println!();
}

pub struct RangeMinimumQuery {
    seg: Vec<i64>,
    n: usize,
}

impl RangeMinimumQuery {
    pub fn new(size: usize) -> RangeMinimumQuery {
        let mut m = 1;
        while m <= size {
            m *= 2;
        }
        RangeMinimumQuery {
            seg: vec![MAX; m * 2],
            n: m,
        }
    }

    pub fn update(&mut self, mut k: usize, value: i64) {
        k += self.n - 1;
        self.seg[k] = value;
        while k > 0 {
            k = (k - 1) / 2;
            self.seg[k] = cmp::min(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
        }
    }

    /// Get the minimum value in the array in the range [a, b)
    ///
    /// # Panics
    ///
    /// Panics if `a >= b`.
    pub fn query(&self, a: usize, b: usize) -> i64 {
        assert!(a < b);
        return self.query_range(a, b, 0, 0, self.n);
    }

    pub fn query_range(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if r <= a || b <= l {
            return MAX;
        }
        if a <= l && r <= b {
            return self.seg[k];
        }
        let x = self.query_range(a, b, k * 2 + 1, l, (l + r) / 2);
        let y = self.query_range(a, b, k * 2 + 2, (l + r) / 2, r);
        cmp::min(x, y)
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