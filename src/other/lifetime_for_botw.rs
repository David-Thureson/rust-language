#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::cell::RefCell;
use std::rc::Rc;

pub fn main() {
    build();
}

struct Model<'a> {
    items: Vec<Rc<RefCell<Item<'a>>>>,
}

#[derive(Debug)]
struct Item<'a> {
    label: String,
    price: Option<Price<'a>>,
}

#[derive(Debug)]
struct Price<'a> {
    label: String,
    items: Vec<Rc<RefCell<Item<'a>>>>,
}

impl<'a> Model<'a> {
    fn new() -> Self {
        Model { items: vec![] }
    }

    fn report(&self) {
        let mut price = Price::new("price_report", None);
        for one_item in self.items.iter() {
            // let one_price = RefCell::borrow(item).price.as_ref();
            let one_item_price_items: Option<Vec<Rc<RefCell<Item<'a>>>>> =
                RefCell::borrow(one_item)
                    .price
                    .as_ref()
                    .map(|price| price.items.iter().map(|item| Rc::clone(item)).collect());
            if let Some(items) = one_item_price_items {
                for item in items {
                    price.items.push(item);
                }
            }
        }
    }
}

impl<'a> Item<'a> {
    fn new(label: &str, price: Option<Price<'a>>) -> Self {
        Item {
            label: label.to_string(),
            price,
        }
    }
}

impl<'a> Price<'a> {
    fn new(label: &str, first_item: Option<Rc<RefCell<Item<'a>>>>) -> Self {
        let mut price = Price {
            label: label.to_string(),
            items: vec![],
        };
        if let Some(item) = first_item {
            price.items.push(item);
        }
        price
    }

    fn from(label: &str, other_price: Option<&Price<'a>>) -> Self {
        let mut price = Price {
            label: label.to_string(),
            items: vec![],
        };
        if let Some(other_price) = other_price {
            for item in other_price.items.iter().map(|x| Rc::clone(x)) {
                price.items.push(item);
            }
        }
        price
    }

    fn copy_from(&mut self, other_price: Option<&Price<'a>>) {
        if let Some(other_price) = other_price {
            for item in other_price.items.iter().map(|x| Rc::clone(x)) {
                self.items.push(item);
            }
        }
    }
    //fn copy_from(&mut self, other_price: &Price) {
}

fn build<'a>() {
    let mut model = Model::new();
    model
        .items
        .push(Rc::new(RefCell::new(Item::new("i0", None))));
    model
        .items
        .push(Rc::new(RefCell::new(Item::new("i1", None))));
    let p1 = Price::new("p1", Some(Rc::clone(model.items.get(0).unwrap())));
    // RefCell::borrow_mut(model.items.get_mut(1).unwrap()).price = Some(p1);

    let p2 = Price::from("p2", Some(&p1));

    let p4 = {
        let p3 = Price::new("p3", Some(Rc::clone(model.items.get(0).unwrap())));
        Price::from("p4", Some(&p3))
    };

    let p5 = {
        let i2 = Rc::new(RefCell::new(Item::new("i2", None)));
        Price::new("p5", Some(Rc::clone(&i2)))
    };
    //bg!(&p5);

    model.report();
}

/*
struct A<'a> {
    val: &'a u32,
}

struct B<'a> {
    ref_a: Rc<RefCell<A<'a>>>,
}

struct C<'a> {
    ref_a: Rc<RefCell<A<'a>>>,
}

fn clone_b() {
    // let a = A { val: 1 };
    // let b = B { ref_a: &a };
    // let b2 = b.clone();
}

fn try_copy_from() {
    /*
    let a1 = Rc::new(RefCell::new(A { val: 1 }));
    let a2 = Rc::new(RefCell::new(A { val: 2 }));
    let a3 = Rc::new(RefCell::new(A { val: 2 }));
    let b1 = Rc::new(RefCell::new())
    let c1 = C {
        ref_a: vec![a1.clone()]
    };
    let c2 = C {
        ref_a: vec![a2.clone(), a3.clone()]
    };
    c1.add_from(&c2);
    */
}

fn try_scoped() {
    let a1 = Rc::new(RefCell::new(A::new(1)));
    let b1 = B::new(Rc::clone(&a1));
    let b2 = B::new(Rc::clone(&b1.ref_a));

    let b4 = {
        let b3 = B::new(Rc::clone(&a1));
        B::new(Rc::clone(&b1.ref_a))
    };
}

impl A<'_> {
    pub fn new(val: u32) -> Self {
        A {
            val: &32,
        }
    }
}

impl <'a> B<'a> {
    pub fn new(ref_a: Rc<RefCell<A<'a>>>) -> Self {
        B {
            ref_a,
        }
    }
}

impl <'a> C<'a> {

    pub fn new(ref_a: Rc<RefCell<A<'a>>>) -> Self {
        C {
            ref_a,
        }
    }

    pub fn copy_from(&mut self, other: &C) {

    }
}

*/
