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
    // let source = AutoSource::from("0");
    input!{
        // from source,
        a: [usize; 9],
        n: usize,
        b: [usize; n]
    };

    let mut card = vec![false;9];

    for i in 0..9{
        for j in 0..n{
            if a[i] == b[j] {
                card[i] = true;
            }
        }
    }

    if card[0] & card[1] & card[2]
        || card[3] & card[4] & card[5]
        || card[6] & card[7] & card[8]
        || card[0] & card[3] & card[6]
        || card[1] & card[4] & card[7]
        || card[2] & card[5] & card[8]
        || card[0] & card[4] & card[8]
        || card[2] & card[4] & card[6]{
        println!("Yes")
    }
    else {
        println!("No")
    }

}
