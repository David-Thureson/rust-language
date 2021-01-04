// The Little Book of Rust Macros: https://danielkeep.github.io/tlborm/book/README.html

#![allow(unused_macros)]
#![allow(dead_code)]

pub fn main() {
    // try_simple();
    // try_dead_rule();
    // try_capture_expr_then_stringify();
    // try_capture_then_match_tokens();
    // try_capture_then_what_is();
    // try_syntax_context();
    try_self();
}

// Empty pattern.
macro_rules! four {
    () => {1 + 3};
}

// Match expression.
macro_rules! times_five {
    ($e:expr) => {5 * $e};
}

macro_rules! times_five_block {
    ($e:expr) => {5 * {$e}};
}

macro_rules! vec_strs {
    ($($element:expr),*) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $element));
            )*
            v
        }
    };
}

fn try_simple() {
    let _a = four!();
    let _b = &four!();
    // Won't compile:
    // let _c = four!(0);
    dbg!{four![]};
    dbg!{four!{}};
    dbg!{&four![]};
    dbg!{&four!{}};

    let a = times_five!(3);
    assert_eq!(15, a);
    dbg!(times_five![4]);
    dbg!(times_five!{a / 3});
    dbg!(times_five!{four!() / four!()});
    dbg!({
        let b1 = 4;
        b1 * b1
    });
    dbg!(times_five_block!{a / 5});
    // Doesn't work. Why?
    // dbg!(times_five!{
    //    let b1 = 4;
    //    b1 * b1
    // });
    // dbg!(times_five_block!{
    //    let b1 = 4;
    //    b1 * b1
    //});

    dbg!(vec_strs!("abc", 2, 5 * 4));
}

macro_rules! dead_rule {
    ($e:expr) => { "expr" };
    ($i:ident) => {" ident" };
}

macro_rules! dead_rule_reversed {
    ($i:ident) => {" ident" };
    ($e:expr) => { "expr" };
}

fn try_dead_rule() {
    dbg!(dead_rule!(blue));
    dbg!(dead_rule!(3));
    dbg!(dead_rule!(3 + 2));
    // dbg!(dead_rule!(3 +));
    dbg!(dead_rule_reversed!(blue));
    dbg!(dead_rule_reversed!(3));
    dbg!(dead_rule_reversed!(3 + 2));
}

macro_rules! capture_expr_then_stringify {
    ($e:expr) => {
        stringify!($e)
    };
}

fn try_capture_expr_then_stringify() {
    // This doesn't match what the book shows.
    println!("{:?}", stringify!(dummy(2 * (1 + (3)))));
    println!("{:?}", capture_expr_then_stringify!(dummy(2 * (1 + (3)))));
}

macro_rules! capture_then_match_tokens {
    ($e:expr) => { match_tokens!($e)};
}

macro_rules! match_tokens {
    ($a:tt + $i:ident)  => { format!("add with identifier: {} + {}; {} + {}", $a, $i, stringify!($a), stringify!($i)) };
    ($a:tt + $b:tt)     => { format!("addition: {} + {}; {} + {}", $a, $b, stringify!($a), stringify!($b)) };
    (($i:ident))        => { format!("identifier: {}", $i) };
    ($($other:tt)*)     => { concat!("something else (", stringify!($($other)*), ")") };
}

fn try_capture_then_match_tokens() {
    let x = 7;
    println!("{}\n{}\n{}\n{}\n",
             match_tokens!((x)),
             match_tokens!(3 + x),
             match_tokens!(x + 3),
             match_tokens!(x, 8, 13));
    // trace_macros!(true);
    println!("{}\n{}\n{}\n",
            capture_then_match_tokens!((caravan)),
            capture_then_match_tokens!(3 + 6),
            capture_then_match_tokens!(5));
    // trace_macros!(false);
}

macro_rules! capture_then_what_is {
    (#[$m:meta]) => {what_is!(#[$m])};
}

macro_rules! what_is {
    (#[no_mangle]) => { "no_mangle attribute" };
    (#[inline]) => { "inline attribute" };
    ($($tts:tt)*) => { concat!("something else (", stringify!($($tts)*), ")") };
}

fn try_capture_then_what_is() {
    println!("{}\n{}\n{}\n{}\n",
             what_is!(#[no_mangle]),
             what_is!(#[inline]),
             capture_then_what_is!(#[no_mangle]),
             capture_then_what_is!(#[inline]));
}

macro_rules! using_a_broken {
    ($e:expr) => {
        {
            let a = 42;
            $e
        }
    }
}

macro_rules! using_a {
    ($a:ident, $e:expr) => {
        {
            let $a = 42;
            $e
        }
    }
}

fn try_syntax_context() {
    // Won't compile:
    // let four = using_a_broken!(a / 10);
    let four = using_a!(a, a / 10);
    dbg!(four);
}

macro_rules! what_is {
    (self) => { "the keyword 'self'" };
    ($i:ident) => { concat!("the identifier '", stringify!($i), "'" ) };
}

// Example of taking a macro as an argument and expanding it.
macro_rules! call_with_ident {
    ($c:ident($i:ident)) => { $c!($i) };
}

fn try_self() {
    println!("{}", what_is!(self));
    // Note that this line is not calling what_is!() and sending the result to call_with_ident!() but instead calling
    // call_with_ident!() with something that will match the pattern so that $c is the identifier named "what_is".
    println!("{}", call_with_ident!(what_is(self)));
}
/*
macro_rules! make_mutable {
    ($i:ident) => { let mut $i = $i; };
}
*/

macro_rules! make_mutable {
    ($i:ident) => {let mut $i = $i;};
}

macro_rules! double_method {
    ($self_:ident, $body:expr) => {
        fn double(mut $self_) -> Dummy {
            $body
        }
    };
}

struct Dummy(i32);

impl Dummy {
    // trace_macros!(true);
    double_method! {self, {
        self.0 *= 2;
        self
    }}
    // trace_macros!(false);

    fn a(self) -> Self {
        self.double()
    }
}

// Example of a recursive macro. In this case it eventually evaluates to nothing so it's only interesting with
// trace_macros!() turned on.
macro_rules! each_tt {
    () => {};
    ($_tt:tt $($rest:tt)*) => { each_tt!($($rest)*); };
}

//trace_macros!(true);
each_tt!(abc 123 red green blue);
//trace_macros!(false);

/*
macro_rules! sing {
    () => {};
    ($tt:tt $($rest:tt)*) => {log_syntax!($tt); sing!($($rest)*);};
}

sing! {
    ^ < @ < . @ *
    '\x08' '{' '"' _ # ' '
    - @ '$' && / _ %
    ! ( '\t' @ | = >
    ; '\x08' '\'' + '$' ? '\x7f'
    , # '"' ~ | ) '\x07'
}
*/

