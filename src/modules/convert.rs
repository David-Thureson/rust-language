#![allow(unused_imports)]

use std::convert::identity;

pub fn main() {
    try_identity();
}

fn try_identity() {
    let v = vec![Some(1), None, Some(3)];
    dbg!(&v);
    // let a = v.iter().filter_map::<i32, i32>(identity).collect::<Vec<_>>();
    // dbg!(&a);
    //dbg!(iter.filter(identity).collect::<Vec<_>>());
}
