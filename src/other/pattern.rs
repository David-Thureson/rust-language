#![allow(dead_code)]

pub fn main() {
    // try_match_guard();
    try_reference();
}

fn try_reference() {
    let a = &4;
    dbg!(match a {
        // By matching on a reference, within the match guard [if x < 5] we have a value.
        &x if x < 5 => format!("value < 5: {}", x),
        &x => format!("other value: {}", x),
    });

    let a = Some(A { x: 4 });
    dbg!(match &a {
        // Not necessary to say [&Some]:
        Some(b) if b.x < 5 => format!("value < 5: {}", b.x),
        Some(b) => format!("other value: {}", b.x),
        None => "no value".to_string(),
    });
    dbg!(a.as_ref().unwrap().x);

    // As above but match on [a] rather than [&a].
    let a = Some(A { x: 4 });
    dbg!(match a {
        // Without the [ref] this gives "error[E0008]: cannot bind by-move into a pattern guard".
        Some(ref b) if b.x < 5 => format!("value < 5: {}", b.x),
        Some(b) => format!("other value: {}", b.x),
        None => "no value".to_string(),
    });
    // Won't compile sinc a was moved into the match.
    // dbg!(a.as_ref().unwrap().x);
}

fn try_match_guard() {
    // let a: Option<i32> = None;
    let a = Some(4);

    dbg!(match a {
        Some(val) if val < 5 => format!("value < 5: {}", val),
        Some(val) => format!("any value: {}", val),
        None => "no value".to_string(),
    });

    // No match guard in [if let] or [while let]:
    // if let Some(val) if val < 5 {
    //     println!("branch for <5: {}, val");
    //}

    let a = Some(A { x: 4 });

    dbg!(match a {
        // Without the [ref] this gives "error[E0008]: cannot bind by-move into a pattern guard".
        // Don't use something like:
        //   Some(&b)
        // because that would mean matching on a reference and getting the value while [ref] means
        // matching on a value and getting a reference.
        Some(ref b) if b.x < 5 => format!("value < 5: {}", b.x),
        Some(ref b) if b.x < 10 => format!("value < 10: {}", b.x),
        Some(b) => format!("any value: {}", b.x),
        None => "no value".to_string(),
    });
}

struct A {
    x: i32,
}
