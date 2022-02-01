#![allow(incomplete_features)]

#![feature(try_trait)]
#![feature(box_syntax)]
#![feature(option_result_contains)]
#![feature(option_expect_none)]
#![feature(option_unwrap_none)]
#![feature(cell_update)]
#![feature(slice_partition_at_index)]
#![feature(slice_partition_dedup)]
// #![feature(repeat_generic_slice)]
#![feature(specialization)]
#![feature(int_error_matching)]
// #![feature(bool_to_option)]
#![feature(trace_macros)]
#![feature(log_syntax)]
#![feature(bool_to_option)]

extern crate util;
// pub use util::*;

pub mod language_util;
//pub use language_util::*;

pub mod challenges;
pub mod modules;
pub mod other;
pub mod other_crates;
pub mod traits;
pub mod types;

// pub use closures::*;
// pub use enum_type::*;
// pub use mem_module::*;
// pub use operators::*;
// pub use option_type::*;
// pub use parallel_iterators::*;
// pub use pointers::*;
// pub use slice_type::*;

//pub use binary_tree::*;

// pub use pointer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
