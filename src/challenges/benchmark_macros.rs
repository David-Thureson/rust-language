#![allow(unused_macros)]
#![allow(dead_code)]

fn main() {

}

macro_rules! quicksort_rayon { () => { quicksort_rayon::quicksort_rayon(v, QUICKSORT_RAYON_MIN_SPLIT_SIZE, QUICKSORT_RAYON_MIN_THREAD_SIZE); } }
macro_rules! vec_sort_unstable { () => { v.sort_unstable(); } }

macro_rules! gb_3 {
    ($e:ident, $c:ident) => {
        $e(stringify!($c));
    }
}

macro_rules! gb_4 {
    ($change_var:ident, $c:ident) => {
        group.bench_with_input(BenchmarkId::new(stringify!($c), $change_var), &$change_var, |b, &$change_var| {
            b.iter_batched_ref(|| data_func(count), |v| { $c!() }, BatchSize::LargeInput)
         });
    }
}


fn abc(s: &str) {

}

fn a() {
    // gb_3!(abc, abc);
    gb_4!(count, quicksort_rayon);
}
