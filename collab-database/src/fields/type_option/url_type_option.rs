use crate::entity::FieldType;
use crate::error::DatabaseError;
use crate::fields::{StringifyTypeOption, TypeOptionData, TypeOptionDataBuilder};
use crate::rows::{new_cell_builder, Cell};
use crate::template::entity::CELL_DATA;
use collab::preclude::Any;
use collab::util::AnyMapExt;
use serde::{Deserialize, Serialize};
use yrs::encoding::serde::from_any;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct URLTypeOption {
  #[serde(default)]
  pub url: String,
  #[serde(default)]
  pub content: String,
}

impl StringifyTypeOption for URLTypeOption {
  fn stringify_text(&self, text: &str) -> String {
    text.to_string()
  }
}

impl From<TypeOptionData> for URLTypeOption {
  fn from(data: TypeOptionData) -> Self {
    from_any(&Any::from(data)).unwrap()
  }
}

impl From<URLTypeOption> for TypeOptionData {
  fn from(data: URLTypeOption) -> Self {
    TypeOptionDataBuilder::from([
      ("url".into(), data.url.into()),
      ("content".into(), data.content.into()),
    ])
  }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct URLCellData {
  pub data: String,
}

impl AsRef<str> for URLCellData {
  fn as_ref(&self) -> &str {
    &self.data
  }
}

impl URLCellData {
  pub fn new(s: &str) -> Self {
    Self {
      data: s.to_string(),
    }
  }

  pub fn to_json(&self) -> Result<String, DatabaseError> {
    serde_json::to_string(self).map_err(|err| DatabaseError::Internal(err.into()))
  }
}

impl From<&Cell> for URLCellData {
  fn from(cell: &Cell) -> Self {
    Self {
      data: cell.get_as(CELL_DATA).unwrap_or_default(),
    }
  }
}

impl From<URLCellData> for Cell {
  fn from(data: URLCellData) -> Self {
    let mut cell = new_cell_builder(FieldType::URL);
    cell.insert(CELL_DATA.into(), data.data.into());
    cell
  }
}

impl ToString for URLCellData {
  fn to_string(&self) -> String {
    self.to_json().unwrap()
  }
}
