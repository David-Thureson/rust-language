// https://doc.rust-lang.org/std/primitive.slice.html
// https://doc.rust-lang.org/std/slice/index.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

// use rand::prelude::*;
// use rand::{thread_rng, Rng};
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use std::cmp::Ord;
use std::convert::From;
use std::fmt::{self, Debug};
use std::mem::size_of;
use std::ops::*;
use std::slice::from_mut;
use std::slice::from_ref;
use std::time::Instant;

pub fn main() {
    // try_binary_search();
    // try_chunks_exact();
    // try_chunks_mut();
    // try_concat();
    // try_first();
    // try_first_mut();
    // try_get();
    // try_get_mut();
    // try_partition_at_index();
    // try_partition_dedup();
    // try_partition_dedup_by();
    // try_rotate_left_and_rotate_right();
    // try_split();
    // try_splitn();
    // try_split_at();
    // try_split_first();
    // try_split_first_mut();
    // try_swap();
    // try_windows();

    // try_mutable_slices();
    // try_slice_concat_ext();
    // try_from();
    // try_as_ptr();

    try_pointer_speed();

    // test_make_random_vec();

    // let x: Box<usize> = box 8;
}

fn try_binary_search() {
    let a = &mut [10, 20, 30, 40, 50];
    dbg!(&a.binary_search(&40));
    dbg!(&a.binary_search(&15));

    // Insert values while maintaining sorted order.
    let mut b = vec![10, 20, 30, 40, 50];
    insert_maintain_order(&mut b, 15);
    insert_maintain_order(&mut b, 55);
    insert_maintain_order(&mut b, 30);
    insert_maintain_order(&mut b, 5);
    dbg!(&b);

    // Insert only new values while maintaining sorted order.
    let mut c = vec![10, 20, 30, 40, 50];
    insert_unique_maintain_order(&mut c, 15);
    insert_unique_maintain_order(&mut c, 20);
    insert_unique_maintain_order(&mut c, 15);
    insert_unique_maintain_order(&mut c, 12);
    dbg!(&c);
}

fn try_chunks() {
    let a = &mut [10, 20, 30, 40, 50];
    a.chunks(3).for_each(|x| {
        dbg!(x);
    });
}

fn try_chunks_exact() {
    let a = &mut [10, 20, 30, 40, 50];
    let a_iter = a.chunks_exact(2);
    dbg!(&a_iter.remainder());
    a_iter.for_each(|x| {
        dbg!(x);
    });
}

fn try_chunks_mut() {
    let a = &mut [10, 20, 30, 40, 50];
    a.chunks_mut(3).for_each(|x| {
        dbg!(x);
    });

    let a = &mut [10, 20, 30, 40, 50];
    let mut a_iter = a.chunks_mut(3);
    let c_1 = a_iter.next();
    let c_2 = a_iter.next();
    let c_3 = a_iter.next();
    // Won't compile.
    // dbg!(&a);
    dbg!(&c_1, &c_2, &c_3);
    c_1.map(|x| x[1] += 5);
    c_2.map(|x| x[0] += 5);
    // Won't compile. c_1 and c_2 were moved into map() above.
    // dbg!(&c_1, &c_2);
    dbg!(&a);
}

fn try_concat() {
    // Think of this as flattening.
    dbg!(["abc", "defg"].concat());
    dbg!([[1, 2], [3, 4], [5, 6]].concat());
    let a = &[1, 2];
    let b = &[3, 4, 5];
    // let c = &[a, b];
    // dbg!(c.concat());
    dbg!([vec!(1, 2), vec!(3, 4, 5)].concat());
}

fn try_first() {
    let a = &[10, 20, 30, 40, 50];
    dbg!(&a.first());
    let a_first_1 = a.first();
    let a_first_2 = a.first();
}

fn try_first_mut() {
    let a = &mut [10, 20, 30, 40, 50];
    dbg!(&a.first());
    // let a_first_1 = a.first();
    let a_first_mut = a.first_mut();
    dbg!(&a_first_mut);
    // Won't compile: "cannot borrow `a` as immutable because it is also borrowed as mutable".
    // dbg!(&a);
    // This consumes a_first_mut so in the next line we can see a again.
    a_first_mut.map(|x| *x = 15);
    dbg!(&a);

    // let a_first_1 = a.first();
}

fn try_get() {
    // Either declaration for a works. The first is an array and the second is a reference to a slice.
    // With explicit types they would be:
    //   let a: [usize; 5] = [10, 20, 30, 40, 50];
    //   let a: &[usize] = &[10, 20, 30, 40, 50];
    // let a = [10, 20, 30, 40, 50];
    let a = &[10, 20, 30, 40, 50];
    dbg!(&a);

    let b: &usize = a.get(2).unwrap();
    dbg!(&b);

    let c: &usize = a.get(2).unwrap();
    dbg!(&a);
    dbg!(&b);
    dbg!(&c);

    let d: &[usize] = a.get(3..=4).unwrap();
    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
    dbg!(&d);

    // Range that's only partially in bounds. Returns None.
    let e = a.get(3..=6);
    dbg!(&e);
}

fn try_get_mut() {
    // Either declaration for a works. See try_get().
    let a = &mut [10, 20, 30, 40, 50];
    // let mut a = [10, 20, 30, 40, 50];
    dbg!(&a);

    let b: &mut usize = a.get_mut(2).unwrap();
    // dbg!(&a);
    dbg!(&b);

    let c: &mut [usize] = a.get_mut(3..=4).unwrap();
    // dbg!(&a);
    // At this point b can no longer be used even though the ranges don't overlap.
    // dbg!(&b);
    dbg!(&c);

    c[0] = 70;
    dbg!(&c);
}

fn try_partition_at_index() {
    let mut a = make_random_vec(20, 11, 20);
    dbg!(&a);
    let (before_index, at_index, after_index) = a.select_nth_unstable(8);
    dbg!(&before_index, &at_index, &after_index);
    dbg!(&a);
}

fn try_partition_dedup() {
    let mut a = make_random_vec(20, 11, 15);
    dbg!(&a);
    let (first, second) = a.partition_dedup();
    dbg!(&first, &second);
    dbg!(&a);
}

fn try_partition_dedup_by() {
    let mut a = make_random_vec(30, 101, 200);
    dbg!(&a);
    // In the first partition, don't allow consecutive even or consecutive odd numbers.
    let (first, second) = a.partition_dedup_by(|a, b| *a % 2 == *b % 2);
    // Slight variation using partititon_dedup_by_key(). Same results.
    // let (first, second) = a.partition_dedup_by_key(|a| *a % 2);
    dbg!(&first, &second);
    dbg!(&a);
}

fn try_rotate_left_and_rotate_right() {
    let a = &mut [10, 20, 30, 40, 50, 60];
    dbg!(&a);
    a.rotate_left(2);
    dbg!(&a);
    a[1..=3].rotate_right(1);
    dbg!(&a);
}

fn try_split() {
    let a = &[11, 22, 30, 40, 55, 60];
    let split = a.split(|x| x % 10 == 0);
    dbg!(&split);
    split.for_each(|x| {
        dbg!(x);
    });
}

fn try_splitn() {
    let a = &[11, 22, 30, 40, 55, 60];
    let split = a.splitn(3, |x| x % 10 == 0);
    split.for_each(|x| {
        dbg!(x);
    });
}

fn try_split_at() {
    let a = &[10, 20, 30, 40, 50];
    let (first, second) = a.split_at(2);
    dbg!(&a, &first, &second);
}

fn try_split_first() {
    let a = &[10, 20, 30, 40, 50];
    let (first, rest) = a.split_first().unwrap();
    dbg!(&first);
    dbg!(&rest);
}

fn try_split_first_mut() {
    let a = &mut [10, 20, 30, 40, 50];
    a[4] = 55;
    let (first_mut, rest_mut) = a.split_first_mut().unwrap();
    // Can't get a reference to a because we use first_mut and second_mut later.
    // dbg!(&a);
    dbg!(&first_mut, &rest_mut);
    *first_mut = 12;
    rest_mut[3] = 45;
    dbg!(&first_mut);
    dbg!(&rest_mut);
    // Getting a reference to a drops first_mut and second_mut.
    dbg!(&a);
    // Getting the reference to a invalidated first_mut and second_mut.
    // dbg!(&first_mut);
}

fn try_swap() {
    let a = &mut [10, 20, 30, 40, 50];
    a.swap(3, 0);
    dbg!(&a);
}

fn try_windows() {
    let a = &mut [10, 20, 30, 40, 50];
    a.windows(3).for_each(|x| {
        dbg!(x);
    });

    let a = &mut [10, 20, 30, 40, 50];
    let mut a_iter = a.windows(3);
    let w_1 = a_iter.next();
    let w_2 = a_iter.next();
    let w_3 = a_iter.next();
    dbg!(&a, &w_1, &w_2, &w_3);
}

fn try_pointer_speed() {
    let a = (0..=1_000_000).into_iter().collect::<Vec<usize>>();

    // For the array, 100,000 works but one million leads to "thread 'main' has overflowed its stack".
    // let a: [usize; 100_000] = [0; 100_000];

    let a_len = a.len();

    // Loop with indexing.
    let start = Instant::now();
    let mut sum = 0;
    let mut i = 0;
    while i < a_len {
        sum += a[i];
        i += 1;
    }
    println!(
        "\nLoop with indexing: sum = {}, time = {:?}",
        sum,
        start.elapsed()
    );

    // Loop with get().
    let start = Instant::now();
    let mut sum: usize = 0;
    let mut i = 0;
    while i < a_len {
        sum += *a.get(i).unwrap();
        i += 1;
    }
    println!(
        "\nLoop with get(): sum = {}, time = {:?}",
        sum,
        start.elapsed()
    );

    // Loop with get_unchecked().
    let start = Instant::now();
    let mut sum: usize = 0;
    let mut i = 0;
    unsafe {
        while i < a_len {
            sum += a.get_unchecked(i);
            i += 1;
        }
    }
    println!(
        "\nLoop with get_unchecked(): sum = {}, time = {:?}",
        sum,
        start.elapsed()
    );

    // Iterator.
    let start = Instant::now();
    let sum = a.iter().sum::<usize>();
    println!("\nIterator: sum = {}, time = {:?}", sum, start.elapsed());

    // Pointers.
    let start = Instant::now();
    let a_ptr: *const usize = a.as_ptr();
    let mut sum = 0;
    let mut i: isize = 0;
    let a_len = a_len as isize;
    unsafe {
        while i < a_len {
            sum += *a_ptr.offset(i);
            i += 1;
        }
    }
    println!("\nPointers: sum = {}, time = {:?}", sum, start.elapsed());
}

fn try_as_ptr() {
    let a = &mut [10, 20, 30, 40, 50];
    let a_ptr: *const usize = a.as_ptr();
    let a_ptr_mut: *mut usize = a.as_mut_ptr();
    dbg!(&a_ptr);
    // let size = size_of::<usize>() as isize;
    unsafe {
        dbg!(*a_ptr);
        let a_ptr_2: *const usize = a_ptr.offset(1);
        dbg!(*a_ptr.offset(1));
        dbg!(*a_ptr.offset(2));
        dbg!(*a_ptr.offset(3));
        dbg!(*a_ptr_mut);
        // There are two pointers to a, one mutable and one non-mutable.
        *a_ptr_mut = 150;
        dbg!(*a_ptr_mut);
        dbg!(*a_ptr);
        *a_ptr_mut.offset(4) = 170;
    }
    dbg!(&a);
}

fn try_from() {
    let mut a = 20;
    let b: &mut [usize] = from_mut(&mut a);
    dbg!(&a);
    // dbg!(&b);
    a = 30;
    dbg!(&a);

    let mut c = 40;
    let d: &[usize] = from_ref(&c);
    dbg!(&c);
    dbg!(&d);
    c = 50;
    dbg!(&c);
    // dbg!(&d);
}

/*
fn try_slice_concat_ext() {

}

#[derive(Debug)]
struct IntRange {
    start: usize,
    end: usize
}

impl<IntRange> SliceConcatExt<str> for IntRange {
    type Output: String;

    fn concat(&self) -> Self::Output {
        join(self, "")
    }

    fn join(&self, sep: &str) -> Self::Output {
        (self.start..=self.end).to_iter().join(sep)
    }

}
*/

fn try_mutable_slices() {
    let a = &mut [1, 2, 3];
    println!("a = {:?}", &a);

    a[1] = 4;
    println!("a = {:?}", &a);
    assert_eq!(a, &[1, 4, 3]);

    let mut b = a.clone();
    println!("a = {:?}; b = {:?}", &a, &b);
    assert_eq!(&b, &[1, 4, 3]);

    b[0] = 9;
    println!("a = {:?}; b = {:?}", &a, &b);
    assert_eq!(a, &[1, 4, 3]);
    assert_eq!(&b, &[9, 4, 3]);

    let c = &mut b[..];
    // println!("a = {:?}; b = {:?}; c = {:?}", &a, &b, &c);

    // Changes both b and c.
    c[2] = 12;
    for x in &mut *c {
        *x += 5;
    }
    println!("a = {:?}; c = {:?}", &a, &c);
    // println!("a = {:?}; b = {:?}; c = {:?}", &a, &b, &c);
    // assert_eq!(&b, &[9, 4, 12]);
    // assert_eq!(c, &[9, 4, 12]);

    // Changes both b and c. This line can't run if the println! after it is uncommented.
    b[0] = 15;
    // println!("a = {:?}; b = {:?}; c = {:?}", &a, &b, &c);
    // assert_eq!(&b, &[15, 4, 12]);
    // assert_eq!(c, &[15, 4, 12]);
}

fn insert_maintain_order<T: Ord>(vec: &mut Vec<T>, val: T) -> usize {
    let index = vec.binary_search(&val).unwrap_or_else(|x| x);
    vec.insert(index, val);
    index
}

fn insert_unique_maintain_order<T: Ord>(vec: &mut Vec<T>, val: T) -> usize {
    match vec.binary_search(&val) {
        Ok(index) => index,
        Err(index) => {
            vec.insert(index, val);
            index
        }
    }
}

fn test_make_random_vec() {
    dbg!(make_random_vec(100, 11, 20));
}

fn make_random_vec<T: SampleUniform>(count: usize, low: T, high: T) -> Vec<T> {
    let mut vec = Vec::with_capacity(count);
    let between = Uniform::from(low..high);
    let mut rng = rand::thread_rng();
    for i in 0..count {
        vec.push(between.sample(&mut rng));
    }
    vec
}
