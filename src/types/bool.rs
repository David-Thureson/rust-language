#![allow(unused_variables)]

use crate::language_util;

pub fn main() {
    try_then();
}

fn try_then() {
    dbg!(false.then_some(5));
    dbg!(true.then_some(5));

    dbg!(false.then(|| language_util::produce_value(6)));
    dbg!(true.then(|| language_util::produce_value(7)));
}

// Implement the methods shown on https://doc.rust-lang.org/beta/std/primitive.bool.html that seem
// not to be here any more.
trait BoolThen {
    fn then<T, F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> T;

    fn then_some<T>(&self, t: T) -> Option<T>;
}

impl BoolThen for bool {
    fn then<T, F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        match self {
            true => Some(f()),
            false => None,
        }
    }

    fn then_some<T>(&self, val: T) -> Option<T> {
        match self {
            true => Some(val),
            false => None,
        }
    }
}
