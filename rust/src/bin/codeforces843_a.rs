use std::io;
use std::str;
use std::usize;
use std::collections::BTreeMap;


fn main() {
    let n = read::<usize>();
    let x = read_values::<i64>();
    let mut y = x.clone();
    y.sort();

    let mut uf = UnionFind::new(n);
    for i in 0..n {
        match y.binary_search(&x[i]) {
            Ok(to) => uf.unite(i, to),
            Err(_) => panic!("Err"),
        };
    }

    let mut map = BTreeMap::new();
    for i in 0..n {
        let key = uf.find(i);
        if !map.contains_key(&key) {
            map.insert(key.clone(), Vec::new());
        }
        (*map.get_mut(&key).unwrap()).push(i.clone());
    }

    println!("{}", map.len());
    for (_, v) in map.iter() {
        print!("{}", v.len());
        for i in v.iter() {
            print!(" {}", i + 1);
        }
        println!();
    }
}

struct UnionFind {
    parent: Vec<usize>,
    sizes: Vec<usize>,
    size: usize,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).map(|i| { i }).collect::<Vec<usize>>(),
            sizes: vec![1; n],
            size: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            let px = self.parent[x];
            self.parent[x] = self.find(px);
            self.parent[x]
        }
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let fx = self.find(x);
        let fy = self.find(y);
        if fx == fy {
            return false;
        }

        let (tx, ty) = if self.sizes[fx] < self.sizes[fy] {
            (fy, fx)
        } else {
            (fx, fy)
        };

        self.parent[ty] = tx;
        self.sizes[tx] += self.sizes[ty];
        self.sizes[ty] = 0;
        self.size -= 1;
        return true;
    }
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn read<T>() -> T
    where T: std::str::FromStr,
          T::Err: std::fmt::Debug
{
    read_line().trim().parse().unwrap()
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