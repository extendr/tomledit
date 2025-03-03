use crate::{as_value, TomlEditRError};
use extendr_api::prelude::*;
use std::result::Result as Res;
use toml_edit::{ArrayOfTables, InlineTable, Item, Table, Value};

// Tables
pub(crate) fn as_kv_pairs(x: List) -> Res<Vec<(&'static str, Value)>, TomlEditRError> {
    let err = Err(TomlEditRError::CrateError(String::from(
        "Lists must contain only named elements or no named elements",
    )));
    let names = x.names();

    if let Some(mut nm) = names {
        if nm.any(|xi| xi.is_empty()) {
            return err;
        }
    }
    let kvs = x
        .into_iter()
        .filter_map(|(nm, obj)| {
            let v = as_value(obj);
            match v {
                Ok(vv) => Some((nm, vv)),
                Err(_) => None,
            }
        })
        .collect::<Vec<_>>();

    Ok(kvs)
}

pub(crate) fn as_inline_table(x: List) -> Res<InlineTable, TomlEditRError> {
    let kvs = as_kv_pairs(x)?;
    Ok(InlineTable::from_iter(kvs))
}

pub(crate) fn as_table(x: List) -> Res<Table, TomlEditRError> {
    let kvs = as_kv_pairs(x)?;
    Ok(Table::from_iter(kvs))
}

// Array of Tables
pub(crate) fn as_array_of_tables(x: List) -> Res<ArrayOfTables, TomlEditRError> {
    if !x.inherits("data.frame") {
        return Err(TomlEditRError::CrateError(String::from(
            "data.frame expected",
        )));
    }

    let col_names = match x.names() {
        Some(nm) => nm.collect::<Vec<_>>(),
        None => {
            return Err(TomlEditRError::CrateError(String::from(
                "Column names required",
            )));
        }
    };

    let k = x.len();
    let n = x[0].len();

    let mut arr = ArrayOfTables::new();

    for i in 0..n {
        let mut tbl = Table::new();

        for j in 0..k {
            let col = &x[j];
            match col.rtype() {
                Rtype::Logicals => {
                    let elt = Logicals::try_from(col)?[i];
                    let v = as_value(elt.into());
                    if let Ok(v) = v {
                        tbl.insert(col_names[j], Item::Value(v));
                    }
                }
                Rtype::Integers => {
                    if col.is_factor() {
                        let inner = col.as_str_iter().unwrap().collect::<Strings>()[i].clone();
                        if let Ok(v) = as_value(inner.into()) {
                            tbl.insert(col_names[j], Item::Value(v));
                        }
                    } else {
                        let inner = Integers::try_from(col);
                        let elt = inner?[i];
                        let v = as_value(elt.into());
                        if let Ok(v) = v {
                            tbl.insert(col_names[j], Item::Value(v));
                        }
                    }
                }
                Rtype::Doubles => {
                    let elt = Doubles::try_from(col)?[i];
                    let v = as_value(elt.into());
                    if let Ok(v) = v {
                        tbl.insert(col_names[j], Item::Value(v));
                    }
                }
                Rtype::Strings => {
                    let elt = Strings::try_from(col)?[i].clone();
                    let v = as_value(elt.into());
                    if let Ok(v) = v {
                        tbl.insert(col_names[j], Item::Value(v));
                    }
                }
                Rtype::List => {
                    let elt = List::try_from(col)?[i].clone();
                    let v = as_value(elt.into());
                    if let Ok(v) = v {
                        tbl.insert(col_names[j], Item::Value(v));
                    }
                }
                _ => continue,
            }
        }
        arr.push(tbl);
    }

    Ok(arr)
}
