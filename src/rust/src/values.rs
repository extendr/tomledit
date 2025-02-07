// Values
use crate::{array::*, parse_date, table::as_inline_table, TomlEditRError};
use extendr_api::prelude::*;
use std::result::Result as Res;
use toml_edit::{Formatted, Value};

pub(crate) fn as_value_string(x: Strings) -> Res<Value, TomlEditRError> {
    if x.len() > 1 {
        return Err(TomlEditRError::CrateError(String::from(
            "Character vector with more than 1 element cannot be represented as a scalar value",
        )));
    }

    if x.len() == 0 {
        return Err(TomlEditRError::CrateError(String::from(
            "Empty vector found",
        )));
    }

    let inner = x.elt(0);
    if inner.is_na() {
        return Err(TomlEditRError::CrateError(String::from(
            "Cannot serialize a missing value to toml",
        )));
    }
    let fstr = Formatted::new(x.elt(0).to_string());

    Ok(Value::String(fstr))
}

pub(crate) fn as_value_int(x: Integers) -> Res<Value, TomlEditRError> {
    if x.len() > 1 {
        return Err(TomlEditRError::CrateError(String::from(
            "Vector with more than 1 element cannot be represented as a scalar value",
        )));
    }

    if x.len() == 0 {
        return Err(TomlEditRError::CrateError(String::from(
            "Empty vector found",
        )));
    }

    let inner = x.elt(0);
    if inner.is_na() {
        return Err(TomlEditRError::CrateError(String::from(
            "Cannot serialize a missing value to toml",
        )));
    }

    Ok(Value::Integer(Formatted::new(inner.inner() as i64)))
}

pub(crate) fn as_value_dbl(x: Doubles) -> Res<Value, TomlEditRError> {
    if x.len() > 1 {
        return Err(TomlEditRError::CrateError(String::from(
            "Vector with more than 1 element cannot be represented as a scalar value",
        )));
    }

    if x.len() == 0 {
        return Err(TomlEditRError::CrateError(String::from(
            "Empty vector found",
        )));
    }
    let inner = x.elt(0);
    if inner.is_na() {
        return Err(TomlEditRError::CrateError(String::from(
            "Cannot serialize a missing value to toml",
        )));
    }

    Ok(Value::Float(Formatted::new(inner.inner())))
}

pub(crate) fn as_value_bool(x: Logicals) -> Res<Value, TomlEditRError> {
    if x.len() > 1 {
        return Err(TomlEditRError::CrateError(String::from(
            "Vector with more than 1 element cannot be represented as a scalar value",
        )));
    }

    if x.len() == 0 {
        return Err(TomlEditRError::CrateError(String::from(
            "Empty vector found",
        )));
    }
    let inner = x.elt(0);
    if inner.is_na() {
        return Err(TomlEditRError::CrateError(String::from(
            "Cannot serialize a missing value to toml",
        )));
    }

    Ok(Value::Boolean(Formatted::new(inner.to_bool())))
}

pub(crate) fn as_value_date(x: Strings) -> Res<Value, TomlEditRError> {
    if x.len() > 1 {
        return Err(TomlEditRError::CrateError(String::from(
            "Vector with more than 1 element cannot be represented as a scalar value",
        )));
    }

    if x.len() == 0 {
        return Err(TomlEditRError::CrateError(String::from(
            "Empty vector found",
        )));
    }
    let inner = x.elt(0);
    Ok(Value::Datetime(Formatted::new(
        parse_date(inner.as_str())
            .map_err(|e| TomlEditRError::OtherError(Box::new(e)))?
            .into(),
    )))
}

pub(crate) fn as_value(x: Robj) -> Res<Value, TomlEditRError> {
    match x.rtype() {
        Rtype::Logicals => {
            let inner = Logicals::try_from(x)?;
            match inner.len() == 1 {
                true => as_value_bool(inner),
                false => Ok(Value::Array(as_array_logicals(inner))),
            }
        }
        Rtype::Integers => {
            if x.is_factor() {
                let inner = x.as_str_iter().unwrap().collect::<Strings>();
                match inner.len() == 1 {
                    true => as_value_string(inner),
                    false => Ok(Value::Array(as_array_strings(inner))),
                }
            } else {
                let inner = Integers::try_from(x)?;
                match inner.len() == 1 {
                    true => as_value_int(inner),
                    false => Ok(Value::Array(as_array_ints(inner))),
                }
            }
        }
        Rtype::Doubles => {
            if x.inherits("Date") | x.inherits("POSIXct") {
                // cast dates to strings and parse those
                let res_strings = R!("format")
                    .expect("failed to access format function from rust")
                    .as_function()
                    .expect("format function must be available")
                    .call(pairlist!(x, "%Y-%m-%d".into_robj()))?;

                let strings = Strings::try_from(res_strings)?;
                let res = match strings.len() == 1 {
                    false => Value::Array(as_array_date(strings)),
                    true => Value::Datetime(Formatted::new(
                        *as_value_date(strings)?.as_datetime().unwrap(),
                    )),
                };
                return Ok(res);
            }
            let inner = Doubles::try_from(x)?;
            match inner.len() == 1 {
                true => as_value_dbl(inner),
                false => Ok(Value::Array(as_array_dbls(inner))),
            }
        }
        Rtype::Strings => {
            let inner = Strings::try_from(x)?;
            match inner.len() == 1 {
                true => as_value_string(inner),
                false => Ok(Value::Array(as_array_strings(inner))),
            }
        }
        Rtype::List => Ok(Value::InlineTable(as_inline_table(List::try_from(x)?)?)),
        Rtype::Rstr => {
            let inner = Rstr::try_from(x)?;
            as_value_string(inner.into())
        }
        _ => Err(TomlEditRError::CrateError(String::from(
            "Unsupported R type",
        ))),
    }
}
