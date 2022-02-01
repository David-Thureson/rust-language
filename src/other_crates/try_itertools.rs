#![allow(dead_code)]

use itertools::Itertools;
use itertools::*;

pub fn main() {
    // try_all_equal();
    // try_assert_equal();
    try_batching();
}

fn try_assert_equal() {
    assert_equal("exceed".split('c'), "exceed".split('c'));
    //bg!("exceed".chars().skip(2).collect::<Vec<_>>(), "ceed".chars().collect::<Vec<_>>());
    assert_equal("exceed".chars().skip(2), "deec".chars().rev());
    // Fails:
    // assert_equal("exceed".split('c'), "excess".split('c'));
}

fn try_all_equal() {
    let data = vec![1, 1, 1, 2, 2, 3, 3, 3, 4, 5, 5];
    assert!(!data.iter().all_equal());
    assert!(data[0..3].iter().all_equal());
    assert!(data[3..5].iter().all_equal());
    assert!(data[5..8].iter().all_equal());

    let data: Option<usize> = None;
    assert!(data.into_iter().all_equal());
}

fn try_batching() {
    //let range = 0..4;
    //let range = 0..7;

    // An adaptor that gathers elements in pairs
    /*
    let pit = range.batching(|it| {
        match it.next() {
            None => None,
            Some(x) => match it.next() {
                None => None,
                Some(y) => Some((x, y)),
            }
        }
    });
    */

    // Sum until a zero is reached.
    let pit = [1, 3, 0, 4, 9, 8, 0, 0, 5].iter().batching(|it| {
        let mut sum = 0;
        let mut no_next = false;
        loop {
            match it.next() {
                None => {
                    no_next = true;
                    break;
                },
                Some(x) => {
                    if *x == 0 {
                        break;
                    } else {
                        sum += *x;
                    }
                }
            }
        }
        if no_next && sum == 0 {
            None
        } else {
            Some(sum)
        }
    });

    dbg!(pit.collect::<Vec<_>>());
}