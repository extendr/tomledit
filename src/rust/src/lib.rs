use extendr_api::prelude::*;
use std::fs::read_to_string;
use toml_edit::DocumentMut;

mod array;
mod error_handling;
mod item;
mod table;
mod to_robj;
mod values;

pub(crate) use array::*;
pub(crate) use error_handling::*;
pub(crate) use item::*;
pub(crate) use table::*;
pub(crate) use to_robj::*;
pub(crate) use values::*;

#[extendr]
#[derive(Clone)]
pub struct Toml(pub DocumentMut);

impl From<DocumentMut> for Toml {
    fn from(value: DocumentMut) -> Self {
        Toml(value)
    }
}

impl From<Toml> for DocumentMut {
    fn from(value: Toml) -> Self {
        value.0
    }
}

// Note that each method creates a clone of itself to ensure
#[extendr]
impl Toml {
    fn new() -> Self {
        Toml(DocumentMut::new())
    }

    fn parse_toml(x: String) -> Result<Self> {
        Ok(Toml(
            x.parse::<DocumentMut>()
                .map_err(|e| TomlEditRError::from(e))?,
        ))
    }

    fn read(path: &str) -> Result<Self> {
        let toml = read_to_string(path).map_err(|e| TomlEditRError::OtherError(Box::new(e)))?;
        Self::parse_toml(toml)
    }

    fn write(&self, path: &str) -> Result<()> {
        let to_write = self.0.to_string();
        Ok(std::fs::write(path, to_write).map_err(|e| TomlEditRError::OtherError(Box::new(e)))?)
    }

    fn format(&self) -> String {
        let res = self.0.to_string();
        res
    }

    fn format_lines(&mut self) -> Strings {
        self.format().split("\n").collect::<Strings>()
    }

    fn insert_list(&self, x: List, df_as_array: bool) -> Result<Self> {
        let mut new = self.clone();

        for (k, v) in x.into_iter() {
            new.0.insert(k, as_item(v, !k.is_empty(), df_as_array)?);
        }

        Ok(new)
    }

    fn remove_item(&self, key: &str) -> Result<Self> {
        let mut new = self.clone();
        new.0.remove(key);
        Ok(new)
    }

    fn get_item(&self, x: Strings) -> Result<Robj> {
        let depth = x.len();
        let err = String::from("Failed to extract item using the provided keys");
        let mut item = self.0.get(&x[0]).ok_or(Error::Other(err.clone()))?;
        for i in 1..depth {
            item = item.get(&*x[i]).ok_or(Error::Other(err.clone()))?;
        }

        Ok(item_to_robj(item))
    }

    fn from_toml(&self) -> Robj {
        item_to_robj(self.0.as_item())
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod tomledit;
    impl Toml;
}
