pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SecuritiesListBankRequestFilterItemValue {
    String(String),

    Double(f64),

    Boolean(bool),

    SecuritiesListBankRequestFilterItemValueThreeItemList(
        Vec<SecuritiesListBankRequestFilterItemValueThreeItem>,
    ),
}

impl SecuritiesListBankRequestFilterItemValue {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_securities_list_bank_request_filter_item_value_three_item_list(&self) -> bool {
        matches!(
            self,
            Self::SecuritiesListBankRequestFilterItemValueThreeItemList(_)
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

    pub fn as_securities_list_bank_request_filter_item_value_three_item_list(
        &self,
    ) -> Option<&Vec<SecuritiesListBankRequestFilterItemValueThreeItem>> {
        match self {
            Self::SecuritiesListBankRequestFilterItemValueThreeItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_securities_list_bank_request_filter_item_value_three_item_list(
        self,
    ) -> Option<Vec<SecuritiesListBankRequestFilterItemValueThreeItem>> {
        match self {
            Self::SecuritiesListBankRequestFilterItemValueThreeItemList(value) => Some(value),
            _ => None,
        }
    }
}
