#![allow(unused_variables)]
#![allow(dead_code)]

// extern crate parallel_iterator;

use rayon::prelude::*;
// use rayon::iter::ParallelIterator;
// use parallel_iterator::ParallelIterator;
// use std::thread;
use std::time;

pub fn main() {
    // try_simple();
    // try_sum();
    try_rayon_sum_of_squares();
    try_rayon_parallel_iterator();
}

fn try_rayon_parallel_iterator() {
    let value_count = 100_000_000;
    let a = (0..value_count).into_iter().collect::<Vec<usize>>();

    /*
    a.par_iter().for_each(|x| {
        if x % 1_000_000 == 0 {
            println!("x = {}", x);
        }
    });
    */

    // Chunks.
    let start = time::Instant::now();
    a.par_chunks(10_000_000).for_each(|x| {
        // println!("x = {:?}", x.iter().min());
        println!("x = {:?}", x.par_iter().min());
    });
    println!("\nchunks: time = {:?}", start.elapsed());
}

fn try_rayon_sum_of_squares() {
    let value_count = 1_000_000_000;
    let a = (0..value_count).into_iter().collect::<Vec<usize>>();

    let start = time::Instant::now();
    /*
    let sum = a.par_iter()
            .map(|&i| i * i)
            .sum();
    */
    let start = time::Instant::now();
    let sum = sum_of_squares_parallel(&a);
    println!(
        "\nrayon sum_of_squares_parallel(): sum = {}, time = {:?}",
        sum,
        start.elapsed()
    );

    let start = time::Instant::now();
    let sum = sum_of_squares_serial(&a);
    println!(
        "\nrayon sum_of_squares_serial(): sum = {}, time = {:?}",
        sum,
        start.elapsed()
    );
}

fn sum_of_squares_parallel(input: &[usize]) -> usize {
    input.par_iter().map(|&i| i * i).sum()
}

fn sum_of_squares_serial(input: &[usize]) -> usize {
    input.iter().map(|&i| i * i).sum()
}

/*
fn try_sum() {
    let value_count = 1_000_000;
    let chunk_size = value_count / 10;

    let a = (0..value_count).into_iter().collect::<Vec<usize>>();

    let start = time::Instant::now();
    let sum = a.iter().sum::<usize>();
    println!("\nIterator: sum = {}, time = {:?}", sum, start.elapsed());

    let r: std::ops::Range<usize> = 0..value_count;
    let i: std::ops::Range<usize> = r.into_iter();

    let start = time::Instant::now();
    let sum = (0..value_count).sum::<usize>();
    println!("\nsum(): sum = {}, time = {:?}", sum, start.elapsed());

    let chunks = (0..value_count).collect::<Vec<_>>().chunks(100_000);

    for chunk in (0..value_count).collect::<Vec<_>>().chunks(100_000) {
        // chunk is a &[usize].
        dbg!(chunk.len());
    }

    let start = time::Instant::now();
    /*
    for chunk_sum in ParallelIterator::new(
        // move || a.chunks(chunk_size),
        // move || (0..value_count).into_iter().chunks(100_000),
        // move || (0..1_000_000).collect::<Vec<_>>().chunks(100_000),
        || chunks,
        || sum_chunk)
    {
        dbg!(&chunk_sum);
    }
    */

    for chunk_sum in ParallelIterator::new(
        // move || a.chunks(chunk_size),
        // move || (0..value_count).into_iter().chunks(100_000),
        // move || (0..1_000_000).collect::<Vec<_>>().chunks(100_000),
        || chunks,
        || sum_chunk)
    {
        dbg!(&chunk_sum);
    }

}

fn sum_chunk (chunk: &[usize]) -> usize {
    chunk.into_iter().sum()
}

fn try_simple() {
    // let delay = time::Duration::from_millis(10);

    let start = time::Instant::now();
    for i in 0u32..100 {
        do_some_work(i);
    }
    dbg!(start.elapsed());

    let start = time::Instant::now();
    for i in ParallelIterator::new(
        || (0u32..100),
        || do_some_work)
    {
        // println!("Got a result: {}!", i);
    }
    dbg!(start.elapsed());
}

fn do_some_work(i: u32) -> u32 {
    let delay = time::Duration::from_millis(10);
    thread::sleep(delay);
    i + 1 // let's pretend this is a heavy calculation
}
*/
