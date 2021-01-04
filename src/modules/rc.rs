#![allow(dead_code)]
#![allow(unused_variables)]
//#![allow(unused_imports)]
//#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(unused_mut)]

use std::any::Any;
use std::rc::Rc;
//use crate::language_util::*;

// Documentation: https://doc.rust-lang.org/std/rc/index.html

pub fn main() {
    // try_rc_clone();
    try_rc_downcast();
    // try_rc_get_mut();
    // try_rc_make_mut();
    // try_rc_ptr_eq();
    // try_rc_try_unwrap();
}

pub fn try_rc_clone() {
    let rc1 = Rc::new(4);
    let rc2 = rc1.clone();
    let rc3 = Rc::clone(&rc1);
    dbg!(Rc::strong_count(&rc1), &rc1, &rc2, &rc3);
}

pub fn try_rc_downcast() {
    let rc1: Rc<dyn Any> = Rc::new("abc".to_string());
    dbg!(rc1.downcast::<String>());
    let rc2: Rc<dyn Any> = Rc::new(4);
    dbg!(rc2.downcast::<String>());
}

pub fn try_rc_get_mut() {
    let mut rc1 = Rc::new(4);
    let rc_mut1 = Rc::get_mut(&mut rc1);
    dbg!(&rc_mut1);
    let rc_mut2 = Rc::get_mut(&mut rc1);
    // dbg!(&rc_mut1);
    dbg!(&rc_mut2);
    let mut rc2 = Rc::clone(&rc1);
    dbg!(Rc::get_mut(&mut rc1));
    dbg!(Rc::get_mut(&mut rc2));

    let mut rc1 = Rc::new(4);
    let rc2 = Rc::new(4);
    let rc1_mut = Rc::get_mut(&mut rc1).unwrap();
    *rc1_mut = 5;
    dbg!(&rc1, &rc2);
}

fn try_rc_make_mut() {
    /*
    let mut rc1 = Rc::new(4);
    let weak1 = Rc::downgrade(&rc1);
    dbg!(&rc1, &weak1.upgrade());
    dbg!(Rc::ptr_eq(&rc1, &weak1.upgrade().unwrap()));
    {
        let rc2 = Rc::make_mut(&mut rc1);
        *rc2 = 5;
        dbg!(&rc2);
    }
    dbg!(&rc1);
    dbg!(&weak1.upgrade());
    */

    let mut rc1 = Rc::new(4);
    let mut rc2 = Rc::clone(&rc1);
    dbg!(Rc::ptr_eq(&rc1, &rc2));
    let weak1 = Rc::downgrade(&rc1);
    dbg!(&rc1, &rc2, &weak1.upgrade());
    dbg!(&weak1.upgrade().map(|x| Rc::ptr_eq(&rc1, &x)));
    dbg!(&weak1.upgrade().map(|x| Rc::ptr_eq(&rc2, &x)));
    *Rc::make_mut(&mut rc1) += 1;
    dbg!(Rc::ptr_eq(&rc1, &rc2));
    dbg!(&rc1, &rc2, &weak1.upgrade());
    // The weak pointer is still associated with rc2.
    dbg!(&weak1.upgrade().map(|x| Rc::ptr_eq(&rc1, &x)));
    dbg!(&weak1.upgrade().map(|x| Rc::ptr_eq(&rc2, &x)));
    *Rc::make_mut(&mut rc2) *= 2;
    dbg!(Rc::ptr_eq(&rc1, &rc2));
    dbg!(&rc1, &rc2, &weak1.upgrade());
    // The weak pointer was disassociated.
    dbg!(&weak1.upgrade().map(|x| Rc::ptr_eq(&rc1, &x)));
    dbg!(&weak1.upgrade().map(|x| Rc::ptr_eq(&rc2, &x)));
}

fn try_rc_ptr_eq() {
    let rc1 = Rc::new(4);
    let rc1_same = Rc::clone(&rc1);
    let rc_other = Rc::new(4);
    dbg!(Rc::ptr_eq(&rc1, &rc1_same));
    dbg!(Rc::ptr_eq(&rc1, &rc_other));
}

fn try_rc_try_unwrap() {
    let rc1 = Rc::new(4);
    dbg!(Rc::try_unwrap(rc1));

    let rc1 = Rc::new(5);
    let rc2 = rc1.clone();
    dbg!(Rc::try_unwrap(rc1));
    // rc1 was moved in the previous line so now there is just one reference.
    dbg!(Rc::try_unwrap(rc2));
}
