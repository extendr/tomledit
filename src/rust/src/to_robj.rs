use extendr_api::prelude::*;
use toml_edit::{Array, Item, Value};
// toml_edit::Value -> R object
// Mapping the toml type to the corresponding R type
pub(crate) fn match_toml_type(type_: &str) -> Rtype {
    match type_ {
        "integer" => Rtype::Integers,
        "string" => Rtype::Strings,
        "datetime" => Rtype::Strings,
        "float" => Rtype::Doubles,
        "boolean" => Rtype::Logicals,
        _ => Rtype::List,
    }
}

// Finding lowest common denominator for the types
pub(crate) fn determine_array_rtype(array: &Array) -> Rtype {
    let mut rt = Rtype::Any;
    let n = array.len();

    if n == 0 {
        return Rtype::Null;
    }

    let mut items = array.iter();
    let first_item = items.next().unwrap();
    let init_type = match_toml_type(first_item.type_name());
    for item in items {
        let next_type = match_toml_type(item.type_name());
        rt = match (&init_type, next_type) {
            (Rtype::Integers, Rtype::Integers) => Rtype::Integers,
            (Rtype::Integers, Rtype::Doubles) => Rtype::Doubles,
            (Rtype::Doubles, Rtype::Integers) => Rtype::Doubles,
            (Rtype::Strings, Rtype::Strings) => Rtype::Strings,
            (Rtype::Logicals, Rtype::Logicals) => Rtype::Logicals,
            _ => Rtype::List,
        };
    }

    rt
}

pub(crate) fn value_to_robj(x: &Value) -> Robj {
    match x {
        Value::String(formatted) => {
            let x = Rstr::from(formatted.value().to_string());
            Strings::from(x).into_robj()
        }
        Value::Integer(formatted) => {
            let ints = Integers::from_values([Rint::from(*formatted.value() as i32)]);
            ints.into_robj()
        }
        Value::Float(formatted) => {
            Doubles::from_values([Rfloat::from(*formatted.value())]).into_robj()
        }
        Value::Boolean(formatted) => {
            Logicals::from_values([Rbool::from(*formatted.value())]).into_robj()
        }
        Value::Datetime(formatted) => {
            let v = *formatted.value();
            Strings::from_values([v.to_string()]).into_robj()
        }
        Value::Array(array) => {
            // Flatten the array to the appropriate kind if we cann
            let rt = determine_array_rtype(array);

            // this is really inefficient but its not the worst
            let res = array
                .into_iter()
                .map(|xi| value_to_robj(xi))
                .collect::<Vec<_>>();

            match rt {
                Rtype::Null => ().into_robj(),
                Rtype::Logicals => res
                    .into_iter()
                    .map(|xi| Rbool::try_from(xi).unwrap())
                    .collect::<Logicals>()
                    .into_robj(),
                Rtype::Integers => res
                    .into_iter()
                    .map(|xi| Rint::try_from(xi).unwrap())
                    .collect::<Integers>()
                    .into_robj(),
                Rtype::Doubles => res
                    .into_iter()
                    .map(|xi| Rfloat::try_from(xi).unwrap())
                    .collect::<Doubles>()
                    .into_robj(),
                Rtype::Strings => {
                    // FIXME this is HORRIBLY inefficient
                    let all_strings = res
                        .into_iter()
                        .flat_map(|xi| {
                            Strings::try_from(xi)
                                .unwrap()
                                .into_iter()
                                .map(|i| i.to_string())
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<String>>();
                    Strings::from_values(all_strings).into_robj()
                }
                Rtype::List => List::from_values(res).into_robj(),
                // everything else gets cast into a lsit
                _ => List::from_values(res).into_robj(),
            }
        }
        // tables we recurse
        Value::InlineTable(inline_table) => {
            let mut names = Vec::new();
            inline_table
                .into_iter()
                .map(|(k, v)| {
                    names.push(k);
                    value_to_robj(v)
                })
                .collect::<List>()
                .set_names(names)
                .cloned()
                .into_robj()
        }
    }
}

pub(crate) fn item_to_robj(x: &Item) -> Robj {
    match x {
        Item::None => ().into_robj(),
        Item::Value(value) => value_to_robj(value),
        Item::Table(table) => {
            let mut names = Vec::new();
            table
                .into_iter()
                .map(|(k, v)| {
                    names.push(k);
                    item_to_robj(v)
                })
                .collect::<List>()
                .set_names(names)
                .cloned()
                .into_robj()
        }
        Item::ArrayOfTables(array_of_tables) => array_of_tables
            .into_iter()
            .map(|tbl| {
                let mut names = Vec::new();
                tbl.into_iter()
                    .map(|(k, v)| {
                        names.push(k);
                        item_to_robj(v)
                    })
                    .collect::<List>()
                    .set_names(names)
                    .cloned()
                    .into_robj()
            })
            .collect::<List>()
            .into_robj(),
    }
}
