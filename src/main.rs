#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

// Rust Standard Library: https://doc.rust-lang.org/std/index.html

use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use typename::TypeName;

use language::*;
use modules::*;
// use crate::rc_module::*;
// use crate::any_module::*;

fn main() {
    println!("\nLanguage\n");
    // challenges::binary_tree_vector::main();
    // challenges::binary_tree_nested::main();
    // challenges::parallel_iterator::main();
    // challenges::use_trait_if_found::main();
    // modules::any::main();
    // modules::cell::main();
    // modules::convert::main();
    // modules::future::main();
    modules::mem::main();
    // modules::rc::main();
    // other::lifetime_for_botw::main();
    // other::macros::main();
    // other::operator::main();
    // other::pattern::main();
    // other::write_macros::main();
    // types::bool::main();
    // types::closure::main();
    // types::enum_type::main();
    // types::function::main();
    // types::option::main();
    // types::pointer::main();
    // types::result::main();
    // types::slice::main();
    // types::string::main();
    println!("\nLanguage done\n");
}

/*
TODO:

All of the collections.

*/