pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ReceiptsListPosRequestFilterItemValue {
    String(String),

    Double(f64),

    Boolean(bool),

    ReceiptsListPosRequestFilterItemValueThreeItemList(
        Vec<ReceiptsListPosRequestFilterItemValueThreeItem>,
    ),
}

impl ReceiptsListPosRequestFilterItemValue {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_receipts_list_pos_request_filter_item_value_three_item_list(&self) -> bool {
        matches!(
            self,
            Self::ReceiptsListPosRequestFilterItemValueThreeItemList(_)
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

    pub fn as_receipts_list_pos_request_filter_item_value_three_item_list(
        &self,
    ) -> Option<&Vec<ReceiptsListPosRequestFilterItemValueThreeItem>> {
        match self {
            Self::ReceiptsListPosRequestFilterItemValueThreeItemList(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_receipts_list_pos_request_filter_item_value_three_item_list(
        self,
    ) -> Option<Vec<ReceiptsListPosRequestFilterItemValueThreeItem>> {
        match self {
            Self::ReceiptsListPosRequestFilterItemValueThreeItemList(value) => Some(value),
            _ => None,
        }
    }
}
