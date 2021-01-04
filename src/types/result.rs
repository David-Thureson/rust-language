#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::convert::TryInto;
use std::error::Error;
use std::num::{IntErrorKind, NonZeroI32, ParseIntError};

pub fn main() {
    try_question_mark_operator();
}

fn try_question_mark_operator() {
    dbg!(result_parseinterror("abc"));
    dbg!(result_parseinterror("123"));

    dbg!(result_box_error(None));
    dbg!(result_box_error(Some("abc")));
    dbg!(result_box_error(Some("0")));
    dbg!(result_box_error(Some("123")));
}

fn result_box_error(val: Option<&str>) -> Result<String, Box<dyn Error>> {
    // let val = val.ok_or("val was None".to_string())?;
    // let val = i32::from_str_radix(val, 10)?;
    // let val = NonZeroI32::new(val).ok_or("val was zero")?;
    // Ok(format!("value is {}", val))
    Ok(format!(
        "value is {}",
        NonZeroI32::new(i32::from_str_radix(
            val.ok_or("val was None".to_string())?,
            10
        )?)
        .ok_or("val was zero")?
    ))
}

fn result_parseinterror(val: &str) -> Result<String, ParseIntError> {
    // let val = i32::from_str_radix(val, 10)?;
    // Ok(format!("value is {}", val))
    Ok(format!("value is {}", i32::from_str_radix(val, 10)?))
}

/*
fn returns_result(val: Option<&str>) -> Result<String, String> {
    // let val = val.ok_or("val was None".to_string())?;
    let val = val.ok_or("abc".to_string())?;
    let val = i32::from_str_radix(val, 10)?;
    // let val = (val as u32)?;
    let val = NonZeroI32::new(val).ok_or(ParseIntError{ kind: IntErrorKind::Zero })?;
    Ok(format!("value is {}", val))
    // into_result, try_into
    // i32::from_str_radix(val.unwrap()?, 10)?
}
*/
