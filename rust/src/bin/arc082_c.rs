use std::io;
use std::str;
use std::usize;
use std::cmp;

fn mod_pow(x: i64, mut exp: i64, modulo: i64) -> i64 {
    let mut result = 1;
    let mut cur = x;
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * cur) % modulo;
        }
        cur = (cur * cur) % modulo;
        exp >>= 1;
    }
    result
}

fn main() {
    let modulo = 998244353;
    let n = read_values::<usize>()[0];
    let (x, y) = {
        let mut x = vec![0; n];
        let mut y = vec![0; n];
        for i in 0..n {
            let v = read_values::<i64>();
            x[i] = v[0];
            y[i] = v[1];
        }
        (x, y)
    };

    let mut ans = mod_pow(2, n as i64, modulo);
    ans -= 1;
    ans -= n as i64;
    ans = (ans + modulo) % modulo;

    // 両端がそれぞれ i, j となるような線分に乗っている点の数を数える
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = x[i] - x[j];
            let dy = y[i] - y[j];

            let mut count = 0;
            let mut left = i;
            let mut right = j;
            for k in 0..n {
                let kx = x[i] - x[k];
                let ky = y[i] - y[k];
                if dx * ky - kx * dy == 0 {
                    left = cmp::min(left, k);
                    right = cmp::max(right, k);
                    count += 1;
                }
            }

            // 両端が i, j でない時はスルー
            if left != i || right != j {
                continue;
            }

            // 凸包の面積が 0 であるような部分集合のサイズ
            let subset = mod_pow(2, count, modulo) - count - 1;
            ans = (ans + modulo - subset) % modulo;
        }
    }

    println!("{}", ans);
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