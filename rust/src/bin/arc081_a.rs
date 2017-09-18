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
use std::collections::VecDeque;


fn main() {
    let mut sc = Scanner::new();
    let n = sc.parse::<usize>();
    let mut a: Vec<u64> = (0..n).map(|_| { sc.parse::<u64>() }).collect::<Vec<_>>();
    a.sort();

    let mut q = VecDeque::new();
    for i in (0..n).rev() {
        q.push_back(a[i]);
    }

    let mut ans = 1;
    let mut count = 0;
    while q.len() > 1 && count < 2 {
        let v1 = q.pop_front().unwrap();
        if *q.front().unwrap() == v1 {
            count += 1;
            ans *= v1;
            q.pop_front();
        }
    }
    if count < 2 {
        ans = 0;
    }
    println!("{}", ans);
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