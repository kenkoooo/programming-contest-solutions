use std::io;
use std::str;
use std::usize;
use std::cmp;

fn main() {
    let mut random = Xorshift32::new(114514);
    let (n, start, x) = {
        let v = read_values::<i32>();
        (v[0] as usize, (v[1] - 1) as usize, v[2])
    };
    assert!(n > 0);

    let mut value = vec![-1; n];
    let mut next = vec![0; n];

    let init = cmp::min(n, 1000);
    assert!(init > 0);
    let mut k;
    for _ in 0..init {
        k = start;
        while value[k] != -1 {
            k = random.next() as usize % n;
        }
        assert!(value[k] == -1);

        println!("? {}", (k + 1));
        let input = read_values::<i32>();
        value[k] = input[0];
        next[k] = input[1];
        assert!(value[k] >= 0);
    }

    k = start;
    for i in 0..n {
        if value[k] < value[i] && value[i] <= x {
            k = i;
        }
    }

    for _ in 0..1000 {
        if value[k] >= x {
            println!("! {}", value[k]);
            return;
        }
        if next[k] == -1 {
            println!("! -1");
            return;
        }

        k = (next[k] - 1) as usize;
        println!("? {}", (k + 1));
        let input = read_values::<i32>();
        value[k] = input[0];
        next[k] = input[1];
    }
    println!("! {}", value[k]);
}

struct Xorshift32 {
    state: u32,
}

impl Xorshift32 {
    fn new(seed: u32) -> Xorshift32 {
        Xorshift32 {
            state: seed,
        }
    }

    fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
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