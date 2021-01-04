#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use std::convert::From;
use std::fmt::{self, Debug};
use std::ops::*;

pub fn main() {
    // try_complex();
    // try_wrap_vec();
    // try_shift();
    // try_try();
}

/*
fn try_try() {
}

struct TryWrapper(usize);

impl Try for TryWrapper {
    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self.0 % 2 {
            0 => Self::Ok(),
            _ => Self::Err()
        }
    }

    fn from_error(v: Self::Error) -> Self {

    }

    fn from_ok(v: Self::Ok) -> Self {

    }
}
*/

fn try_shift() {
    let iv = IntVec::from((1..=12).into_iter().collect::<Vec<usize>>());
    // dbg!(&*iv);
    // dbg!(&*(&iv << 2));
    println!("{:?}", *iv);
    println!("{:?}", *(&iv << 2));
}

#[derive(Debug, Clone)]
struct IntVec(Vec<usize>);

impl Deref for IntVec {
    type Target = Vec<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// This is not a real implementation of Shl, just a test of operator overloading.
impl Shl<usize> for &IntVec {
    type Output = IntVec;

    fn shl(self, steps: usize) -> IntVec {
        let mut a = self.clone();
        a.0.rotate_left(steps);
        a
    }
}

impl From<Vec<usize>> for IntVec {
    fn from(v: Vec<usize>) -> Self {
        Self { 0: v }
    }
}

/*
impl Clone for IntVec {
    fn clone(&self) -> {
        let v = self.0.iter().collect::<Vec<usize>>();
        Self {
            0: v
        }
    }
}
*/

fn try_wrap_vec() {
    let v = VecBool::from(vec![
        true, true, false, true, false, false, false, false, true,
    ]);
    dbg!(&v);

    // Test the Deref by calling a method on Vec.
    dbg!(&v.iter().count());

    // Use the debug call for Vec<bool> instead of VecBool by dereferencing.
    println!("{:?}", *v);

    // Get the logical negation.
    dbg!(!v);
}

#[derive(Debug)]
struct VecBool(Vec<bool>);
// struct VecBool {
//	v: Vec<bool>,
//}

impl From<Vec<bool>> for VecBool {
    fn from(v: Vec<bool>) -> Self {
        Self { 0: v }
    }
}

impl Deref for VecBool {
    type Target = Vec<bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Not for VecBool {
    type Output = Self;

    fn not(self) -> Self {
        VecBool::from(self.iter().map(|x| !x).collect::<Vec<bool>>())
    }
}

fn try_complex() {
    let a = c(5.0, -3.0);
    dbg!(&a);

    let mut b = c(10.0, 20.0);

    // Add with references.
    dbg!(&a + &b);

    b += &c(100.0, 200.0);
    dbg!(&b);

    // Add without references.
    dbg!(a + b);

    let mut d = c(2.0, 4.0);

    // AddAssign with reference.
    d += &c(1.0, 1.0);
    dbg!(&d);

    // AddAssign without reference.
    d += c(5.0, 5.0);
    dbg!(&d);

    // Add with reference for first operand.
    let e = &c(1.0, 1.0) + c(2.0, 2.0);
    dbg!(&e);

    // Add with reference for second operand.
    let f = c(3.0, 3.0) + &c(4.0, 4.0);
    dbg!(&f);

    // Add an f64 to a complex number, where the f64 is interpreted as the real part.
    dbg!(c(2.0, 2.0) + 3.5);

    // As above but with a reference to the complex value.
    dbg!(&c(2.0, 2.0), 8.0);

    // As above but with references to both values.
    let add_val = 9.0;
    dbg!(&c(2.0, 2.0) + &add_val);
}

fn c(r: f64, i: f64) -> Complex {
    Complex::new(r, i)
}

fn i(i: f64) -> Complex {
    Complex::new(0.0, i)
}

struct Complex
//where T: Add + AddAssign + Div + DivAssign + Mul + MulAssign + Neg + Sub + SubAssign
{
    r: f64,
    i: f64,
}

impl Complex {
    fn new(r: f64, i: f64) -> Self {
        Self { r, i }
    }
}

impl Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let i_desc = if self.i.is_sign_positive() {
            format!("+ {}i", self.i)
        } else {
            format!("- {}i", -self.i)
        };
        write!(f, "{} {}", self.r, i_desc)
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}

impl Add<&Complex> for Complex {
    type Output = Complex;

    fn add(self, other: &Complex) -> Complex {
        Complex {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}

impl Add<Complex> for &Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}

impl Add<&Complex> for &Complex {
    type Output = Complex;

    fn add(self, other: &Complex) -> Complex {
        Complex {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, other: Complex) {
        *self = Complex {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}

impl AddAssign<&Complex> for Complex {
    fn add_assign(&mut self, other: &Complex) {
        *self = Complex {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }
}

impl Add<f64> for Complex {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            r: self.r + other,
            i: self.i,
        }
    }
}

impl Add<f64> for &Complex {
    type Output = Complex;

    fn add(self, other: f64) -> Complex {
        Complex {
            r: self.r + other,
            i: self.i,
        }
    }
}

impl Add<&f64> for &Complex {
    type Output = Complex;

    fn add(self, other: &f64) -> Complex {
        Complex {
            r: self.r + other,
            i: self.i,
        }
    }
}
