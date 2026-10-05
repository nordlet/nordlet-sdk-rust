pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ExchangeRatesListReferenceRequestFilterItemValue {
    String(String),

    Double(f64),

    Boolean(bool),

    ExchangeRatesListReferenceRequestFilterItemValueThreeItemList(
        Vec<ExchangeRatesListReferenceRequestFilterItemValueThreeItem>,
    ),
}

impl ExchangeRatesListReferenceRequestFilterItemValue {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_exchange_rates_list_reference_request_filter_item_value_three_item_list(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ExchangeRatesListReferenceRequestFilterItemValueThreeItemList(_)
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

    pub fn as_exchange_rates_list_reference_request_filter_item_value_three_item_list(
        &self,
    ) -> Option<&Vec<ExchangeRatesListReferenceRequestFilterItemValueThreeItem>> {
        match self {
            Self::ExchangeRatesListReferenceRequestFilterItemValueThreeItemList(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_exchange_rates_list_reference_request_filter_item_value_three_item_list(
        self,
    ) -> Option<Vec<ExchangeRatesListReferenceRequestFilterItemValueThreeItem>> {
        match self {
            Self::ExchangeRatesListReferenceRequestFilterItemValueThreeItemList(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}
