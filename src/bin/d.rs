use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

use petgraph::unionfind::UnionFind;

fn main() {
    // let source = AutoSource::from("4 4 1
    // 2 1
    // 1 3
    // 3 2
    // 3 4
    // 4 1");

    // let source = AutoSource::from("10 9 3
    // 10 1
    // 6 7
    // 8 2
    // 2 5
    // 8 4
    // 7 3
    // 10 9
    // 6 4
    // 5 8
    // 2 6
    // 7 5
    // 3 1
    // ");
    input!{
        // from source,
        n: usize,
        m: usize,
        k: usize,
        ab: [(usize, usize);m],
        cd: [(usize, usize);k],
    };

    let mut tree = UnionFind::new(n);
    for (a, b) in ab.iter() {
        tree.union(*a-1, *b-1);
    }

    let mut sizes = vec![0;n];

    for i in 0..n{
        // println!("{}", tree.find(i))
        sizes[tree.find(i)] += 1;
    }

    let mut ans = vec![0;n];

    for i in 0..n{
        ans[i] = sizes[tree.find(i)] - 1
    }

    for (a, b) in ab.iter(){
        ans[*a-1] -= 1;
        ans[*b-1] -= 1;
    }

    for (c, d) in cd.iter(){
        if tree.equiv(*c-1, *d-1) {
            ans[*c-1] -= 1;
            ans[*d-1] -= 1;
        }
    }

    let mut s = ans.iter().map(|&x| format!("{}", x)).collect::<Vec<String>>().join(" ");
    println!("{}", s)
}
