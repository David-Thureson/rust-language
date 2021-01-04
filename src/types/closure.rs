// http://smallcultfollowing.com/babysteps/blog/2018/04/24/rust-pattern-precise-closure-capture-clauses/

#![allow(dead_code)]

// Simplest closure: || 5

use std::collections::HashMap;

pub fn main() {
    // try_precise_closure_capture()
}

// From // http://smallcultfollowing.com/babysteps/blog/2018/04/24/rust-pattern-precise-closure-capture-clauses/
struct Context {
    input: HashMap<String, usize>,
    output: Vec<usize>,
}

impl Context {
    // Doesn't compile due to "immutable borrow later used by call".
    /*
    fn process_1(&mut self, values: &[String]) {
        self.output.extend(
            values
                .iter()
                .map(|v| self.input.get(v).cloned().unwrap_or(0)),
        );
    }
    */

    fn process_2(&mut self, values: &[String]) {
        for v in values {
            self.output.push(self.input.get(v).cloned().unwrap_or(0));
        }
    }

    // This version works because the closure captures the local variable input rather than self.
    fn process_3(&mut self, values: &[String]) {
        let input = &self.input;
        self.output
            .extend(values.iter().map(|v| input.get(v).cloned().unwrap_or(0)));
    }

    // Variation of the above where the precise capture happens inside the closure definition rather than
    // at the top of the function. Note that there are nested closures.
    fn process_4(&mut self, values: &[String]) {
        self.output.extend(values.iter().map({
            let input = &self.input;
            move |v| input.get(v).cloned().unwrap_or(0)
        }));
    }
}

fn try_precise_closure_capture() {}
