#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt::{self, Debug, Display, Formatter};

pub fn main() {
    // try_display();
    // try_where_for_method();
    try_specialization();
}

struct A {
    val: i32,
}

struct B {
    val: i32,
}

trait Show {
    fn show(&self);
}

impl<T> Show for T {
    default fn show(&self) {
        println!("Default");
    }
}

impl<T> Show for T
where
    T: Display,
{
    fn show(&self) {
        println!("Display: {}", &self);
    }
}

impl Show for A {
    fn show(&self) {
        println!("A val = {}", self.val);
    }
}

fn try_specialization() {
    B { val: 4 }.show();
    5.show();
    A { val: 7 }.show();
}

/*
fn show<T>(val: T)
{
    println!("{}", "no add function");
}

fn show<T>(val: T)
    // where T: std::ops::Add + Debug {
    where T: Copy,
          T: std::ops::Add,
          <T as std::ops::Add>::Output: Debug
{
    // val + val;
    dbg!(val + val);
}
*/

/*
fn try_where_for_method() {
    show(5);
    show("abc".to_string());
}

struct StructDisplay {
    val: i32,
}

struct StructNotDisplay {
    val: i32,
}

impl Display for StructDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "val = {}", self.val)
    }
}

fn try_display() {

}
*/

/*
trait DisplayOrDefault<T> {
    fn display_string(&self) -> String;
}

impl <T> DisplayOrDefault<T> for T {
    fn display_string(&self) -> String {
        "{no display string}".to_string()
    }
}

impl <T: Display> DisplayOrDefault<T> for T {
    fn display_string(&self) -> String {
        format!("{}", &self)
    }
}
*/
