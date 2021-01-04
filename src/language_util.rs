#![macro_use]

/*

use std::fmt::Display;

// These are macros and helper functions specific to the language project. We already import util
// which is shared among crates/projects.

// Note that this uses $x:ty instead of $x:expr. Watch out for this if copying and pasting.
macro_rules! show_size_align {
    ( $( $x:ty ),* ) => {
        {
            $(
                println!("{:<30}: size = {:>2}; align = {:>2}; Option size = {:>2}",
                    stringify!($x),
                    std::mem::size_of::<$x>(),
                    std::mem::align_of::<$x>(),
                    std::mem::size_of::<Option<$x>>(),
                );
            )*
        }
    };
}

// Return the value passed. This is used to show whether a function call is eager- or
// lazy-evaluated.
pub fn produce_value<T: Display>(val: T) -> T {
    println!("produce_value({})", val);
    val
}

// Return the value passed. This is used to show whether a function call is eager- or
// lazy-evaluated.
pub fn produce_value_label<T, L: Display>(val: T, label: L) -> T {
    println!("produce_value({})", label);
    val
}

// Return the boolean value passed. This is used to show whether a function call is eager- or
// lazy-evaluated.
pub fn produce_bool<L: Display>(val: bool, label: L) -> bool {
    println!("produce_value({}) = {}", label, val);
    val
}

pub fn vec_copy_for_display<T: Display>(vec: &Vec<T>) -> Vec<String> {
    vec.iter().map(|x| format!("{}", x)).collect()
}

*/
