#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]

use std::cell::*;

// Documentation: https://doc.rust-lang.org/std/cell/

pub fn main() {
    // try_assert_error_with_thread();
    // try_cell();
    try_refcell();
}

struct A {
    val: i32,
}

fn try_cell() {
    let a = Cell::new(4);
    // a is immutable.
    a.set(5);
    dbg!(&a);

    let b = Cell::new(6);
    dbg!(&a, &b);
    a.swap(&b);
    dbg!(&a, &b);

    dbg!(&a.replace(7));
    dbg!(&a);
    dbg!(&a.get());
    dbg!(a.into_inner());

    dbg!(b.update(|x| x * 2));

    dbg!(b.as_ptr());

    let mut c = Cell::new(11);
    let c_mut = c.get_mut();
    *c_mut = 12;
    dbg!(&c);

    let mut s = "abc".to_string();
    let d = Cell::from_mut(&mut s);
    d.set("def".to_string());
    dbg!(&s);

    let e = Cell::new("abc");
    dbg!(&e.take());
    dbg!(&e);

    let f: &mut [i32] = &mut [1, 2, 3];
    let g = Cell::from_mut(f);
    let h = g.as_slice_of_cells();
    dbg!(&h);

    let i = Cell::new(&4);
    let val = A { val: 4 };
    let j = Cell::new(&val);
}

fn try_refcell() {
    let a = RefCell::new(4);
    dbg!(a.into_inner());

    let b = RefCell::new(4);
    dbg!(b.replace(5));
    dbg!(&b);

    let c = RefCell::new(4);
    dbg!(c.replace_with(|x| *x * 2));
    dbg!(&c);

    let d = RefCell::new(11);
    dbg!(&c, &d);
    c.swap(&d);
    dbg!(&c, &d);

    let d_ref_1 = d.borrow();
    let d_ref_2 = d.borrow();
    // Panic: "already borrowed: BorrowMutError"
    // c.swap(&d);
    let d_ref_3 = Ref::clone(&d_ref_1);
    dbg!(&d_ref_1, &d_ref_2, &d_ref_3);

    let e = RefCell::new((4, 'a'));
    let e_part = Ref::map(e.borrow(), |x| &x.0);
    dbg!(&e, &e_part);
    drop(e_part);
    // Won't compile if the previous line is commented out: "cannot move out of `e` because it is borrowed".
    drop(e);

    let f = RefCell::new([1, 2, 3, 4]);
    let (f_1, f_2) = Ref::map_split(f.borrow(), |slice| slice.split_at(2));
    dbg!(&f, &f_1, &f_2);

    let g = RefCell::new(4);
    let g_ref_1 = g.borrow();
    let g_ref_2_opt = g.try_borrow();
    let g_ref_3_opt = g.try_borrow_mut();
    dbg!(&g, &g_ref_1, &g_ref_2_opt, &g_ref_3_opt);
    drop(g_ref_1);
    drop(g_ref_2_opt);
    dbg!(g.try_borrow_mut());
    let g_refmut = g.borrow_mut();

    let h = RefCell::new((4, 'a'));
    dbg!(&h);
    let mut h_part = RefMut::map(h.borrow_mut(), |x| &mut x.0);
    *h_part = 5;
    dbg!(&h, &h_part);
    drop(h_part);
    dbg!(&h);
    dbg!(&h.as_ptr());

    let mut i = RefCell::new((4, 'a'));
    dbg!(&i);
    let mut i_mut = i.get_mut();
    dbg!(&i_mut);
    i_mut.0 = 5;
    dbg!(&i_mut);
    drop(i_mut);
    dbg!(&i);
}

fn try_assert_error_with_thread() {
    let result = std::thread::spawn(move || {
        let a = RefCell::new(4);
        let a_ref = a.borrow();
        // This will result in an error. We get the callstack but because we're in a thread the
        // program keeps running.
        let a_mutref = a.borrow_mut();
    })
    .join();

    assert!(result.is_err());
    dbg!(result);

    // Do more work to prove that we're still running.
    try_refcell();
}
