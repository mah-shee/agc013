#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut prev = false;
    let mut next = false;
    let mut count = 1;
    let mut skip = true;
    for i in 0..n - 1 {
        if a[i] == a[i + 1] {
            continue;
        } else if a[i] < a[i + 1] {
            next = true;
        } else {
            next = false;
        }
        if skip {
            skip = false;
            prev = next;
            continue;
        }
        if prev == next {
            continue;
        } else {
            count += 1;
            prev = next;
            skip = true;
        }
    }
    println!("{}", count);
}
