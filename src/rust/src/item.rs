use crate::{as_array_list, as_array_of_tables, as_table, as_value, TomlEditRError};
use extendr_api::prelude::*;
use std::result::Result as Res;
use toml_edit::Item;

// Item wrapper
pub(crate) fn as_item(x: Robj, named: bool, df_as_array: bool) -> Res<Item, TomlEditRError> {
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
            } else if !named {
                let mut xx = as_array_list(x.try_into()?)?;
                xx.decor_mut().set_prefix("\n");

                Ok(Item::Value(toml_edit::Value::Array(xx)))
            } else {
                Ok(Item::Table(as_table(List::try_from(x)?)?))
            }
        }
        _ => Err(TomlEditRError::CrateError(format!(
            "Unsupported R type: {:?}",
            x.rtype(),
        ))),
    }
}
