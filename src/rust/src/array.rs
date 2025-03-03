use extendr_api::prelude::*;
use std::num::ParseIntError;
use toml_edit::{Array, Date};

use crate::as_value;

pub(crate) fn as_array_strings(x: Strings) -> Array {
    let mut arr = Array::new();
    for xi in x.into_iter() {
        if !xi.is_na() {
            arr.push(xi.as_str());
        }
    }
    arr
}

pub(crate) fn as_array_ints(x: Integers) -> Array {
    let mut arr = Array::new();
    for xi in x.into_iter() {
        if !xi.is_na() {
            arr.push(xi.inner() as i64);
        }
    }
    arr
}

pub(crate) fn as_array_dbls(x: Doubles) -> Array {
    let mut arr = Array::new();
    for xi in x.into_iter() {
        if !xi.is_na() {
            arr.push(xi.inner());
        }
    }
    arr
}

pub(crate) fn as_array_logicals(x: Logicals) -> Array {
    let mut arr = Array::new();
    for xi in x.into_iter() {
        if !xi.is_na() {
            arr.push(xi.to_bool());
        }
    }
    arr
}

pub(crate) fn as_array_list(x: List) -> Result<Array> {
    let mut arr = Array::new();
    for (_, xi) in x.into_iter() {
        let mut v = as_value(xi)?;
        v.decor_mut().set_prefix("\n    ");
        arr.push_formatted(v)
    }
    arr.set_trailing("\n");
    Ok(arr)
}

pub(crate) fn parse_date(xi: &str) -> std::result::Result<Date, ParseIntError> {
    let mut splits = xi.split("-");

    let year = splits
        .next()
        .expect("Dates should be formatted as `YYYY-MM-DD`")
        .parse::<u16>()?;

    let month = splits
        .next()
        .expect("Dates should be formatted as `YYYY-MM-DD`")
        .parse::<u8>()?;

    let day = splits
        .next()
        .expect("Dates should be formatted as `YYYY-MM-DD`")
        .parse::<u8>()?;

    Ok(Date { year, month, day })
}

pub(crate) fn as_array_date(x: Strings) -> Array {
    let mut arr = Array::new();
    x.into_iter()
        .for_each(|xi| arr.push(parse_date(xi).expect("Failed to parse date")));
    arr
}
