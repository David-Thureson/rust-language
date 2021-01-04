#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

use std::collections::HashMap;
use std::fmt::Display;
use std::mem;
// use crate::language::*;
use ::util::*;
use std::num::ParseIntError;
// mod language_util;
// use crate::language_util::*;

// Documentation: https://doc.rust-lang.org/std/option/enum.Option.html
// Source: https://github.com/rust-lang/rust/blob/master/src/libcore/option.rs

pub fn main() {
    // try_and();
    // try_and_then();
    // try_as_ref();
    // try_as_mut();
    // try_contains();
    // try_copied();
    // try_expect();
    // try_expect_none();
    // try_filter();
    // try_get_or_insert();
    // try_get_or_insert_with();
    // try_is_some();
    // try_iter();
    // try_iter_mut();
    // try_map_or();
    // try_map_or_else();
    // try_ok_or();
    // try_ok_or_else();
    // try_or();
    // try_or_else();
    try_question_mark_operator();
    // try_replace();
    // try_take();
    // try_unwrap_none();
    // try_unwrap_or();
    // try_unwrap_or_else();
    // try_xor();
    // explore_size();
}

fn try_and() {
    let a = Some(4);
    let b = Some(5);
    let c: Option<i32> = None;
    let d: Option<i32> = None;
    // Try anding a with itself.
    assert_eq!(a, a.and(a));
    assert_eq!(b, a.and(b));
    assert_eq!(a, b.and(a));
    assert_eq!(None, a.and(c));
    assert_eq!(None, c.and(a));
    assert_eq!(None, c.and(d));
}

fn try_and_then() {
    let mut a = Some("aaa");
    assert_eq!(Some(3), a.and_then(|x| Some(x.len())));
    assert_eq!(Some(13), a.and_then(|x| Some(produce_value(x.len() + 10))));

    a = None;
    assert_eq!(None, a.and_then(|x| Some(produce_value(x.len() + 20))));
}

fn try_as_ref() {
    // Without as_ref().
    let a = Some("aaa".to_string());
    let a_len = a.map(|x| x.len());
    dbg!(&a_len);
    // Doesn't compile because a was moved.
    // dbg!(&a);

    // With as_ref().
    let b = Some("bbbb".to_string());
    let b_len = b.as_ref().map(|x| x.len());
    dbg!(&b_len);
    // Works because the as_ref() means b was not moved.
    dbg!(&b);
}

fn try_as_mut() {
    let mut a = Some(4);
    a.as_mut().map(|x| *x = 5);
    dbg!(a);

    let b = Some(5);
    // Won't compile because b is not mutable.
    // b.as_mut().map(|x| *x = 5 );

    let mut c = Some(6);
    let c_as_mut = c.as_mut();
    dbg!(c);
    // This won't compile if we _later_ use c_as_mut.
    c.as_mut().map(|x| *x = 7);
    dbg!(c);
    // c_as_mut.map(|x| *x = 8);
}

fn try_contains() {
    let mut a = Some(4);
    assert!(a.contains(&4));
    assert!(!a.contains(&5));
    a = None;
    assert!(!a.contains(&4));
}

fn try_copied() {
    let a_val = 4;
    let a_opt: Option<&i32> = Some(&a_val);
    let b_opt = a_opt.copied();
    dbg!(a_val, &a_opt, &b_opt);

    let mut c_opt: Option<&i32> = Some(&6);
    let d_opt = c_opt.copied();
    dbg!(&c_opt, &d_opt);
}

fn try_expect() {
    let mut a = Some(4);
    dbg!(a.expect("first try no value"));
    a = None;
    dbg!(a.expect("second try no value"));
}

fn try_expect_none() {
    // let a = -5;
    // dbg!(i32::abs(a));

    let mut m = HashMap::new();
    for i in -10..=10 {
        // Does not panic because there is never a duplicate key and thus insert() always returns
        // None.
        m.insert(i, i).expect_none("duplicate key");
    }

    let mut m: HashMap<i32, i32> = HashMap::new();
    for i in -10..=10 {
        // Panics when it encounters a duplicate key and thus insert() returns a Some.
        m.insert(i32::abs(i), i).expect_none("duplicate key");
    }
}

fn try_filter() {
    let mut a = Some(4);
    dbg!(a.filter(|x| produce_bool(*x >= 4, "a with *x >= 4")));
    dbg!(a.filter(|x| produce_bool(*x == 0, "a with *x == 0")));

    a = None;
    dbg!(a.filter(|x| produce_bool(*x >= 4, "None with *x >= 4")));
    dbg!(a.filter(|x| produce_bool(*x == 0, "None with *x == 0")));
}

fn try_get_or_insert() {
    // Used in the leet_code project in merge_two_lists().

    /*
    // Based on the example from the doc page.
    let mut x = None;
    {
        let y: &mut u32 = x.get_or_insert(5);
        assert_eq!(y, &5);
        *y = 7;
    }
    assert_eq!(x, Some(7));
    x = None;
    // *y = 8;
    */

    let mut a = Some(4);
    {
        let b = a.get_or_insert(produce_value_label(5, "for a.is_some()"));
        dbg!(&b);
        *b = 15;
        dbg!(&b);
    }
    dbg!(&a);

    let mut c: Option<i32> = None;
    {
        let d = c.get_or_insert(produce_value_label(6, "for c.is_none()"));
        dbg!(&d);
        *d = 16;
        dbg!(&d);
    }
    dbg!(&c);
}

fn try_get_or_insert_with() {
    let mut a = Some(4);
    {
        let b = a.get_or_insert_with(|| produce_value_label(5, "for a.is_some()"));
        dbg!(&b);
        *b = 15;
        dbg!(&b);
    }
    dbg!(&a);

    let mut c: Option<i32> = None;
    {
        let d = c.get_or_insert_with(|| produce_value_label(6, "for c.is_none()"));
        dbg!(&d);
        *d = 16;
        dbg!(&d);
    }
    dbg!(&c);
}

fn try_is_some() {
    let a = Some(4);
    let b = a.is_some();
    // is_some() didn't take ownership of a so we can still access it.
    let c = a.unwrap();

    let mut x = Some(4);
    let y = x.is_some();
    x = Some(5);
    let z = x.unwrap();
}

fn try_iter() {
    let mut a = Some(4);
    a.iter().for_each(|x| println!("{}", x));

    a = None;
    a.iter().for_each(|x| println!("{}", x));
}

fn try_iter_mut() {
    let mut a = Some(4);
    a.iter_mut().for_each(|x| *x += 10);
    dbg!(&a);
}

fn try_map_or() {
    let a = Some("aaa");
    dbg!(a.map_or(10, |x| x.len()));

    let b = None;
    // The x in the closure needs an explicit type.
    dbg!(b.map_or(10, |x: &mut str| x.len()));

    let c: Option<&str> = None;
    // The x in the closure doesn't need an explicit type.
    dbg!(c.map_or(10, |x| x.len()));
}

fn try_map_or_else() {
    let a = Some("aaa");
    // Show that it's lazy-evaluated; produce_value() is not called.
    dbg!(a.map_or_else(|| produce_value(10), |x| x.len()));

    let b: Option<&str> = None;
    dbg!(b.map_or_else(|| produce_value(11), |x| x.len()));
}

fn try_ok_or() {
    let a = Some(4);
    dbg!(a.ok_or("error with a"));
    // Show that it's eager-evaluated.
    dbg!(a.ok_or(produce_value("error with a")));

    let b: Option<i32> = None;
    dbg!(b.ok_or("error with b"));
    dbg!(b.ok_or(produce_value("error with b")));
}

fn try_ok_or_else() {
    let a = Some(4);
    // Show that it's lazy-evaluated.
    dbg!(a.ok_or_else(|| produce_value("error with a")));

    let b: Option<i32> = None;
    dbg!(b.ok_or_else(|| produce_value("error with b")));
}

fn try_or() {
    let a = Some("aaa");
    let b = Some("bbbb");
    let c: Option<&str> = None;
    let d: Option<&str> = None;
    // Try oring a with itself.
    assert_eq!(a, a.or(a));
    assert_eq!(a, a.or(b));
    // Eager evaluated.
    assert_eq!(a, a.or(Some(produce_value("for a"))));
    assert_eq!(b, b.or(a));
    assert_eq!(a, a.or(c));
    assert_eq!(a, c.or(a));
    assert_eq!(None, c.or(d));
}

fn try_or_else() {
    let mut a = Some(4);
    // Lazy evaluation.
    assert_eq!(
        a,
        a.or_else(|| produce_value_label(Some(5), "for a.is_some()"))
    );

    a = None;
    assert_eq!(
        Some(6),
        a.or_else(|| produce_value_label(Some(6), "for a.is_none()"))
    );
}

fn try_question_mark_operator() {
    dbg!(return_option(None));
    dbg!(return_option(Some("abc")));
    dbg!(return_option(Some("4")));
}

fn try_replace() {
    let mut a = Some(4);
    let b = a.replace(5);
    dbg!(&a, &b);

    a = None;
    let c = a.replace(6);
    dbg!(&a, &c);
}

fn try_take() {
    let mut a = Some(4);
    let b = a.take();
    dbg!(&a, &b);

    let c = a.take();
    dbg!(&c);
}

fn try_unwrap_or() {
    let a = Some(4);
    dbg!(a.unwrap_or(7));

    let b = None;
    dbg!(b.unwrap_or(8));

    // Show that it's evaluated eagerly (produce_value is called) even though the value is not needed.
    dbg!(a.unwrap_or(produce_value(9)));
}

fn try_unwrap_none() {
    let mut m = HashMap::new();
    for i in -10..=10 {
        // Does not panic because there is never a duplicate key and thus insert() always returns
        // None.
        m.insert(i, i).unwrap_none();
    }

    let mut m: HashMap<i32, i32> = HashMap::new();
    for i in -10..=10 {
        // Panics when it encounters a duplicate key and thus insert() returns a Some.
        m.insert(i32::abs(i), i).unwrap_none();
    }
}

fn try_unwrap_or_else() {
    let a = Some(4);
    // Show that it's lazy evaluated; produce_value is not called because it's not needed.
    dbg!(a.unwrap_or_else(|| produce_value(7)));

    let b = None;
    dbg!(b.unwrap_or_else(|| produce_value(8)));

    let c = None;
    // Use a simple closure instead of a function call.
    dbg!(c.unwrap_or_else(|| 10));
}

fn try_xor() {
    let a = Some("aaa");
    let b = Some("bbbb");
    let c: Option<&str> = None;
    let d: Option<&str> = None;
    // Try xoring a with itself.
    assert_eq!(None, a.xor(a));
    assert_eq!(None, a.xor(b));
    assert_eq!(None, b.xor(a));
    assert_eq!(a, a.xor(c));
    assert_eq!(a, c.xor(a));
    assert_eq!(None, c.xor(d));
}

fn return_option(val: Option<&str>) -> Option<i32> {
    // let val = val?;
    // let val: Result<i32, ParseIntError> = i32::from_str_radix(val, 10);
    // let val = i32::from_str_radix(val, 10).ok()?;
    // Note the question mark after [val] which means it will either provide the inner value (in
    // this case the &str) or return from the function immediately with None. The [ok()?] takes the
    // Result of the string-to-int conversion and either provides the int value or returns from
    // the function with None.
    Some(i32::from_str_radix(val?, 10).ok()? * 2)
}
