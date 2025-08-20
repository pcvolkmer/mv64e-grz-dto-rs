//! This crate provides structs to serialize and deserialize GRZ Metadata DTOs.
//! The base struct is `Metadata`.

#![allow(clippy::needless_doctest_main)]

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
pub use crate::metadata::*;

mod metadata;

#[derive(Debug)]
pub struct SerdeError(String);

impl Display for SerdeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Metadata Serde Error: {}", self.0)
    }
}

impl Error for SerdeError {}

impl FromStr for Metadata {
    type Err = SerdeError;

    /// Deserialize an instance of `Metadata` from a string of JSON text.
    ///
    /// # Example
    ///
    /// ```
    /// use mv64e_grz_dto::Metadata;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     const JSON: &str = include_str!("../tests/example_metadata.json");
    ///
    ///     let mtb_file = Metadata::from_str(JSON).unwrap();
    ///     println!("{:#?}", mtb_file);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// If the conversion fails, an `SerdeError` will be returned.
    fn from_str(value: &str) -> Result<Metadata, SerdeError> {
        serde_json::from_str(value).map_err(|err| SerdeError(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MTB_JSON: &str = include_str!("../tests/example_metadata.json");

    #[test]
    fn should_deserialize_json_string() {
        let data = Metadata::from_str(MTB_JSON);
        assert!(data.is_ok())
    }
}
