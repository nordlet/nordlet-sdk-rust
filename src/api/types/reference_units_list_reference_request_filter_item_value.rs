pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UnitsListReferenceRequestFilterItemValue {
    String(String),

    Double(f64),

    Boolean(bool),

    UnitsListReferenceRequestFilterItemValueThreeItemList(
        Vec<UnitsListReferenceRequestFilterItemValueThreeItem>,
    ),
}

impl UnitsListReferenceRequestFilterItemValue {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_units_list_reference_request_filter_item_value_three_item_list(&self) -> bool {
        matches!(
            self,
            Self::UnitsListReferenceRequestFilterItemValueThreeItemList(_)
        )
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_double(&self) -> Option<&f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_double(self) -> Option<f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            Self::Boolean(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_boolean(self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_units_list_reference_request_filter_item_value_three_item_list(
        &self,
    ) -> Option<&Vec<UnitsListReferenceRequestFilterItemValueThreeItem>> {
        match self {
            Self::UnitsListReferenceRequestFilterItemValueThreeItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_units_list_reference_request_filter_item_value_three_item_list(
        self,
    ) -> Option<Vec<UnitsListReferenceRequestFilterItemValueThreeItem>> {
        match self {
            Self::UnitsListReferenceRequestFilterItemValueThreeItemList(value) => Some(value),
            _ => None,
        }
    }
}
