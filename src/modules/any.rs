#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use std::any::{Any, TypeId};
use std::fmt::Debug;

// Documentation: https://doc.rust-lang.org/std/any/

pub fn main() {
    try_any_downcast_ref_and_is();
    // try_any_downcast_mut();
    // try_typeid_of();
}

fn log<T: Any + Debug>(val: &T) {
    let val_any = val as &dyn Any;
    match val_any.downcast_ref::<String>() {
        Some(s) => {
            println!("String: length = {}: {}", s.len(), s);
        }
        None => {
            println!("{:?}", val);
        }
    }
    dbg!(&val_any.is::<String>());
    dbg!(&val_any.is::<i32>());
}

fn try_any_downcast_ref_and_is() {
    log(&"abc".to_string());
    log(&"def");
    log(&4);
}

fn append_if_string<T: Any + Debug>(val: &mut T, add: String) {
    let val_any = val as &mut dyn Any;
    // let a: Option<&mut String> = val_any.downcast_mut::<String>();
    val_any.downcast_mut::<String>().map(|s| {
        *s = format!("{}{}", s, add);
    });
}

fn try_any_downcast_mut() {
    let mut s1 = "abc".to_string();
    append_if_string(&mut s1, "xyz".to_string());
    dbg!(&s1);
    let mut i1 = 4;
    append_if_string(&mut i1, "xyz".to_string());
    dbg!(&i1);
}

fn compare_typeid<T: ?Sized + Any + Debug>(val: &T) {
    dbg!(val, TypeId::of::<String>() == TypeId::of::<T>());
    dbg!(val, TypeId::of::<String>() == val.type_id());
    dbg!(val, TypeId::of::<i32>() == TypeId::of::<T>());
    dbg!(val, TypeId::of::<i32>() == val.type_id());
}

fn try_typeid_of() {
    compare_typeid(&"abc".to_string());
    compare_typeid(&4);
}
