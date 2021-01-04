#![allow(dead_code)]

use std::cell::RefCell;
use std::ops::DerefMut;

pub fn main() {
    try_lazy_init_cache();
}

struct LazyInitCache<T: Copy> {
    value: Option<T>,
    func: fn() -> T,
}

struct LazyInitCacheInteriorMut<T: Copy> {
    value: RefCell<Option<T>>,
    func: fn() -> T,
}

impl <T: Copy> LazyInitCache<T> {
    fn new(func: fn() -> T) -> Self {
        Self {
            value: None,
            func,
        }
    }

    fn value(&mut self) -> T {
        match &self.value {
            Some(value) => *value,
            None => {
                dbg!("Calculating value.");
                let value = (self.func)();
                self.value = Some(value);
                value
            },
        }
    }
}

impl <T: Copy> LazyInitCacheInteriorMut<T> {
    fn new(func: fn() -> T) -> Self {
        Self {
            value: RefCell::new(None),
            func,
        }
    }

    fn value(&self) -> T {
        let mut borrow = RefCell::borrow_mut(&self.value);
        // .deref_mut();
        // let opt = RefMut::deref_mut(&mut borrow);
        // The above line could also be:
        // let opt = borrow.deref_mut();
        match borrow.deref_mut() {
            Some(value) => *value,
            None => {
                dbg!("Calculating value.");
                let value = (self.func)();
                *borrow = Some(value);
                value
            },
        }
    }
}

fn try_lazy_init_cache() {
    let mut lic = LazyInitCache::new(|| 4);
    dbg!(&lic.value());
    dbg!(&lic.value());
    dbg!(&lic.value());

    let licim = LazyInitCacheInteriorMut::new(|| 5);
    dbg!(&licim.value());
    dbg!(&licim.value());
    dbg!(&licim.value());
}