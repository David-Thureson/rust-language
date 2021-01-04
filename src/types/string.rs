pub fn main() {
    try_as_bytes();
}

fn try_as_bytes() {
    dbg!("abc".to_string().as_bytes());
    // String slice instead of string:
    dbg!("abc".as_bytes());

    let unicode = "\u{211D}\u{212D}";
    dbg!(&unicode);
    dbg!(unicode.as_bytes());
}
