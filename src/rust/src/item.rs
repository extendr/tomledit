use crate::{
    as_array_list, as_array_of_tables, as_table, as_tables, as_value, is_table_like, TomlEditRError,
};
use extendr_api::prelude::*;
use std::result::Result as Res;
use toml_edit::Item;

// Item wrapper
pub(crate) fn as_item(x: Robj, df_as_array: bool) -> Res<Item, TomlEditRError> {
    match x.rtype() {
        Rtype::Null => Ok(Item::None),
        Rtype::Rstr | Rtype::Logicals | Rtype::Integers | Rtype::Strings | Rtype::Doubles => {
            Ok(Item::Value(as_value(x)?))
        }
        Rtype::List => {
            if x.inherits("data.frame") {
                match df_as_array {
                    true => Ok(Item::ArrayOfTables(as_array_of_tables(List::try_from(x)?)?)),
                    false => Ok(Item::Table(as_table(List::try_from(x)?)?)),
                }
            } else if x.names().is_some() {
                // named lists are tables, just like in `as_value()`
                Ok(Item::Table(as_table(List::try_from(x)?)?))
            } else {
                let xx = List::try_from(x)?;

                // a list of named lists has no array representation that keeps
                // the elements distinct, so it becomes an array of tables
                if !xx.is_empty() && (0..xx.len()).all(|i| is_table_like(&xx[i])) {
                    return Ok(Item::ArrayOfTables(as_tables(xx)?));
                }

                Ok(Item::Value(toml_edit::Value::Array(as_array_list(xx)?)))
            }
        }
        _ => Err(TomlEditRError::CrateError(format!(
            "Unsupported R type: {:?}",
            x.rtype(),
        ))),
    }
}
