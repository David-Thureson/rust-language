#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use std::cell::RefCell;
use std::mem;
use std::sync::atomic;
// use language_util::show_size_align;
// use language::*;
use crate::*;
use util::format;

// Documentation: https://doc.rust-lang.org/std/mem/index.html

pub fn main() {
    // compare_align_size();
    // try_align_of();
    // try_discriminant();
    // try_drop();
    // try_size_of();
    // try_size_of_val_align_of_val();
    // test_show_size_align_macro();
    // try_size_of_generic_struct();
}

struct StructForSizeOf1 {
    a: i32,
}

struct StructForSizeOf2 {
    a: bool,
    b: i32,
}

// For an Option<u8> the alignment is 1 byte while the size is 2 bytes.
struct StructForSizeOf3 {
    a: Option<u8>,
    b: Option<u8>,
    c: Option<u8>,
    d: Option<u8>,
}

// Try a potentially inefficient order of fields. Since it's not a C structure Rust will reorder
// the fields so that there is no excess padding.
struct StructRustInefficient {
    a: bool,
    b: i32,
    c: bool,
    d: i32,
}

// Try an inefficient order of fields. Since it's a C structure the order of fields is fixed and
// there's a lot of padding.
#[repr(C)]
struct StructCInefficient {
    a: bool,
    b: i32,
    c: bool,
    d: i32,
}

// Try an efficient order of fields for a non-C structure.
struct StructRustEfficient {
    a: bool,
    b: bool,
    c: i32,
    d: i32,
}

// Try an efficient order of fields for a C structure.
#[repr(C)]
struct StructCEfficient {
    a: bool,
    b: bool,
    c: i32,
    d: i32,
}

// Try a potentially inefficient order of fields. Since it's not a C structure Rust will reorder
// the fields so that the size is 20.
#[repr(C)]
struct StructForSizeOf4 {
    a: bool,
    b: i32,
    c: bool,
    d: i32,
    e: bool,
    f: i32,
    g: bool,
    h: i32,
}

// Try a potentially efficient order of fields.
#[repr(C)]
struct StructForSizeOf5 {
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    e: i32,
    f: i32,
    g: i32,
    h: i32,
}

fn compare_align_size() {
    show_size_align!(bool, char, u8, u16, u32, u64, &bool);
    show_size_align!(
        StructRustInefficient,
        StructCInefficient,
        StructRustEfficient,
        StructCEfficient
    );
}

/*
fn compare_align_size() {
    dbg!(mem::align_of::<bool>());
    dbg!(mem::size_of::<bool>());
    dbg!(mem::align_of::<Option<bool>>());
    dbg!(mem::size_of::<Option<bool>>());
    dbg!(mem::align_of::<Box<bool>>());
    dbg!(mem::size_of::<Box<bool>>());
    dbg!(mem::align_of::<u8>());
    dbg!(mem::size_of::<u8>());
    dbg!(mem::align_of::<Option<u8>>());
    dbg!(mem::size_of::<Option<u8>>());
    dbg!(mem::align_of::<&bool>());
    dbg!(mem::size_of::<&bool>());
    dbg!(mem::align_of::<Option<&bool>>());
    dbg!(mem::size_of::<Option<&bool>>());
    dbg!(mem::align_of::<i32>());
    dbg!(mem::size_of::<i32>());
    dbg!(mem::align_of::<Option<i32>>());
    dbg!(mem::size_of::<Option<i32>>());
    dbg!(mem::align_of::<Box<i32>>());
    dbg!(mem::size_of::<Box<i32>>());
    dbg!(mem::align_of::<&i32>());
    dbg!(mem::size_of::<&i32>());
    dbg!(mem::align_of::<Option<&i32>>());
    dbg!(mem::size_of::<Option<&i32>>());
    dbg!(mem::align_of::<StructForSizeOf1>());
    dbg!(mem::size_of::<StructForSizeOf1>());
    dbg!(mem::align_of::<StructForSizeOf2>());
    dbg!(mem::size_of::<StructForSizeOf2>());
    dbg!(mem::align_of::<StructForSizeOf3>());
    dbg!(mem::size_of::<StructForSizeOf3>());
    dbg!(mem::align_of::<StructRustInefficient>());
    dbg!(mem::size_of::<StructRustInefficient>());
    dbg!(mem::align_of::<StructCInefficient>());
    dbg!(mem::size_of::<StructCInefficient>());
    dbg!(mem::align_of::<StructRustEfficient>());
    dbg!(mem::size_of::<StructRustEfficient>());
    dbg!(mem::align_of::<StructCEfficient>());
    dbg!(mem::size_of::<StructCEfficient>());
}
*/

fn try_align_of() {
    dbg!(mem::align_of::<bool>());
    dbg!(mem::align_of::<Option<bool>>());
    dbg!(mem::align_of::<Box<bool>>());
    dbg!(mem::align_of::<u8>());
    dbg!(mem::align_of::<Option<u8>>());
    dbg!(mem::align_of::<&bool>());
    dbg!(mem::align_of::<Option<&bool>>());
    dbg!(mem::align_of::<i32>());
    dbg!(mem::align_of::<Option<i32>>());
    dbg!(mem::align_of::<Box<i32>>());
    dbg!(mem::align_of::<&i32>());
    dbg!(mem::align_of::<Option<&i32>>());
    dbg!(mem::align_of::<StructForSizeOf1>());
    dbg!(mem::align_of::<StructForSizeOf2>());
}

enum EnumDisc1 {
    A(&'static str),
    B(i32),
    C(i32),
}

fn try_discriminant() {
    let mut a1 = EnumDisc1::A("aaa");
    dbg!(mem::discriminant(&a1));
    // Hold on to the discriminant of a1 to prove that it doesn't keep a reference.
    let a1_disc = mem::discriminant(&a1);

    let a2 = EnumDisc1::A("bbb");
    dbg!(mem::discriminant(&a2));
    assert_eq!(mem::discriminant(&a1), mem::discriminant(&a2));

    let b1 = EnumDisc1::B(4);
    dbg!(mem::discriminant(&b1));
    assert_ne!(mem::discriminant(&a1), mem::discriminant(&b1));

    let b2 = EnumDisc1::B(5);
    dbg!(mem::discriminant(&b2));
    assert_eq!(mem::discriminant(&b1), mem::discriminant(&b2));

    let c1 = EnumDisc1::C(4);
    dbg!(mem::discriminant(&c1));
    assert_ne!(mem::discriminant(&b1), mem::discriminant(&c1));

    let mut c2 = EnumDisc1::C(5);
    dbg!(mem::discriminant(&c2));
    assert_eq!(mem::discriminant(&c1), mem::discriminant(&c2));

    // Make sure we can still use the variables.
    a1 = EnumDisc1::A("bbb");
    c2 = EnumDisc1::C(6);
    dbg!(&a1_disc);
}

fn try_drop() {
    let v = vec![1, 2, 3];
    drop(v);
    // The following line won't compile because v was moved into the call to drop().
    // dbg!(&v);

    let rc = RefCell::new(1);
    let mut borrow_mut = rc.borrow_mut();
    *borrow_mut = 2;
    dbg!(*borrow_mut);

    drop(borrow_mut);
    // Won't compile because the value was moved into drop().
    // dbg!(*borrow_mut);

    // Without the drop() this fails at runtime with "already mutably borrowed: BorrowError".
    let borrow = rc.borrow();
}


fn try_size_of_val_align_of_val() {
    let a = 5;
    let b = 5u8;
    dbg!(mem::size_of_val(&5));
    dbg!(mem::align_of_val(&5));
    dbg!(mem::size_of_val(&5u8));
    dbg!(mem::align_of_val(&5u8));

    let x: [u8; 3] = [0; 3];
    dbg!(&x);
    let y: &[u8] = &x;
    dbg!(mem::size_of_val(&x));
    dbg!(mem::align_of_val(&x));
    dbg!(mem::size_of_val(y));
    dbg!(mem::align_of_val(y));
    // This doesn't have a size that's known at compile time.
    // dbg!(mem::size_of::<[u8]>());
}

fn test_show_size_align_macro() {
    show_size_align!(u8);
}

pub struct DivideParallelSettingsGeneric<I, S, T, N>
where
    I: Into<u32>,
    S: Into<u16>,
    T: Into<u16>,
    N: Into<u16>,
{
    low: I,
    high: I,
    min_split_size: S,
    min_thread_size: T,
    nsec_per_item: N,
    inline_fake_work: bool,
    ordering: atomic::Ordering,
}

fn try_size_of_generic_struct() {
    dbg!(mem::size_of::<
        DivideParallelSettingsGeneric<u32, u16, u16, u16>,
    >());
    dbg!(mem::size_of::<
        DivideParallelSettingsGeneric<u16, u8, u16, u8>,
    >());
    let a = DivideParallelSettingsGeneric {
        low: 0u16,
        high: 10_000u16,
        min_split_size: 20u8,
        min_thread_size: 1_000u16,
        nsec_per_item: 100u8,
        inline_fake_work: true,
        ordering: atomic::Ordering::AcqRel,
    };
    dbg!(mem::size_of_val(&a));
    dbg!(mem::size_of_val(&a.low));
    dbg!(mem::size_of_val(&a.min_split_size));
    let b = DivideParallelSettingsGeneric {
        low: 0u32,
        high: 10_000u32,
        min_split_size: 20u16,
        min_thread_size: 1_000u16,
        nsec_per_item: 100u16,
        inline_fake_work: true,
        ordering: atomic::Ordering::AcqRel,
    };
    dbg!(mem::size_of_val(&b));
    dbg!(mem::size_of_val(&b.low));
    dbg!(mem::size_of_val(&b.min_split_size));

    let high = b.high;
    if (high as u64) < (std::u16::MAX as u64) {
        let high = high as u16;
        dbg!(&high);
    }

    // Test the generic format_count() function in the util crate:
    dbg!(format::format_count(a.low));
    dbg!(format::format_count(b.low));
    dbg!(format::format_count(a.min_split_size));
    dbg!(format::format_count(b.min_split_size));
    println!("{}", format::format_count(b.min_split_size));

    dbg!(a.low as u64);
    dbg!(a.min_split_size as u64);

    let c = DivideParallelSettingsGeneric {
        low: 12u32,
        high: 100u32,
        min_split_size: 10_000u16,
        min_thread_size: 1_000u16,
        nsec_per_item: 100u16,
        inline_fake_work: true,
        ordering: atomic::Ordering::AcqRel,
    };
    dbg!(c.low as u64);
    dbg!(c.min_split_size as u64);
}
