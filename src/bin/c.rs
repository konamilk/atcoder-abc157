use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("1 1
    // 1 0");
    input!{
        // from source,
        n: usize,
        m: usize,
        sc: [(usize, char); m]
    };

    for i in 0..=999{
        let t = format!("{}", i);
        if t.len() != n {
            continue
        }
        let chars = t.as_str().chars().collect::<Vec<char>>();
        let mut flg = true;
        for (s, c) in sc.iter(){
            if *s > t.len(){
                flg = false;
            }

            if chars[*s - 1]  != *c{
                flg = false
            }
        }

        if flg {
            println!("{}", i);
            return
        }
    }

    println!("-1");
}
