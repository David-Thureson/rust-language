#![allow(unused_macros)]
#![allow(dead_code)]

fn main() {

}

macro_rules! quicksort_rayon { () => { quicksort_rayon::quicksort_rayon(v, QUICKSORT_RAYON_MIN_SPLIT_SIZE, QUICKSORT_RAYON_MIN_THREAD_SIZE); } }
macro_rules! vec_sort_unstable { () => { v.sort_unstable(); } }

/*
macro_rules! group_benches {
    // ($change_var:ident, ($($c:ident),*)) => {
    ($change_var:ident, $c:ident) => {
        // $(
            group.bench_with_input(BenchmarkId::new($c, $change_var), &$change_var, |b, &$change_var| {
                b.iter_batched_ref(|| data_func($change_var), |v| { $c!() }, BatchSize::LargeInput)
            });
        // )*
    }
}
*/

/*
        group.bench_with_input(BenchmarkId::new("merge_sort_merge_in_place", count), &count, |b, &count| {
            b.iter_batched_ref(|| data_func(count), |v| { merge_sort::merge_sort_merge_in_place(v); }, BatchSize::LargeInput)
        });
*/

/*
macro_rules! group_benches {
    // ($change_var:ident, ($($c:ident),*)) => {
    ($change_var:ident, $c:ident) => {
        // $(
            concat!("group.bench_with_input(BenchmarkId::new(\"", stringify!($c), "\", ", stringify!($change_var), "), &", stringify!($change_var), ", |b, &", stringify!($change_var), "| {\n",
                "b.iter_batched_ref(|| data_func(", stringify!($change_var), "), |v| { ", $c!(), " }, BatchSize::LargeInput)\n",
            "});")
        // )*
    }
}
*/

macro_rules! gb_1 {
    ($c:ident) => {
        // format!("\"{}\"; {}", stringify!($c), $c!())
        $c; $c!()
    }
}

macro_rules! gb_2 {
    ($e:expr, $c:ident) => {
        // format!("\"{}\"; {}", stringify!($c), $c!())
        stringify!($e)
    }
}

macro_rules! gb_3 {
    ($e:tt, $c:ident) => {
        // format!("\"{}\"; {}", stringify!($c), $c!())
        $e
    }
}

fn a() {
    // trace_macros!(true);
    // group_benches!(min_thread_size, quicksort_rayon);
    // gb_1!(quicksort_rayon);
    // gb_2!("abc", quicksort_rayon);
    gb_3!(quicksort_rayon, quicksort_rayon);
    // trace_macros!(false);
}

