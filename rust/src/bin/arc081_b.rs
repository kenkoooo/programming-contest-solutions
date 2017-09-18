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

static MOD: u64 = 1000000007;

fn main() {
    let mut sc = Scanner::new();
    let n = sc.parse::<usize>();
    let x = (0..2).map(|_| { sc.parse::<String>().into_bytes() }).collect::<Vec<_>>();

    let mut i: usize = 0;
    let mut vertical = false;
    let mut ans = 1;
    while i < n {
        if i == 0 {
            if x[0][i] == x[1][i] {
                ans = 3;
                i += 1;
                vertical = true;
            } else {
                ans = 6;
                i += 2;
                vertical = false;
            }
        } else if x[0][i] == x[1][i] {
            if vertical {
                ans *= 2;
            } else {
                ans *= 1;
            }
            i += 1;
            vertical = true;
        } else {
            if vertical {
                ans *= 2;
            } else {
                ans *= 3;
            }
            i += 2;
            vertical = false;
        }
        ans %= MOD;
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