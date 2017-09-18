use std::io;
use std::io::{Read, Stdin};
use std::str;
use std::str::FromStr;
use std::usize;
use std::fmt::Debug;

fn main() {
    let mut sc = Scanner::new();
    let h = sc.parse::<usize>();
    let w = sc.parse::<usize>();
    let n = sc.parse::<usize>();
    let mut a = (0..n).map(|_| {
        return sc.parse::<i32>();
    }).collect::<Vec<i32>>();

    let mut map = vec![vec![0; w]; h];
    let mut cur = 0;
    for i in 0..h {
        if i % 2 == 0 {
            for j in 0..w {
                a[cur] -= 1;
                map[i][j] = cur + 1;
                if a[cur] == 0 {
                    cur += 1;
                }
            }
        } else {
            for j in (0..w).rev() {
                a[cur] -= 1;
                map[i][j] = cur + 1;
                if a[cur] == 0 {
                    cur += 1;
                }
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{} ", map[i][j])
        }
        println!()
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