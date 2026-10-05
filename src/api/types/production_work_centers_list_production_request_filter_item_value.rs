pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum WorkCentersListProductionRequestFilterItemValue {
    String(String),

    Double(f64),

    Boolean(bool),

    WorkCentersListProductionRequestFilterItemValueThreeItemList(
        Vec<WorkCentersListProductionRequestFilterItemValueThreeItem>,
    ),
}

impl WorkCentersListProductionRequestFilterItemValue {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub fn is_work_centers_list_production_request_filter_item_value_three_item_list(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::WorkCentersListProductionRequestFilterItemValueThreeItemList(_)
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

    pub fn as_work_centers_list_production_request_filter_item_value_three_item_list(
        &self,
    ) -> Option<&Vec<WorkCentersListProductionRequestFilterItemValueThreeItem>> {
        match self {
            Self::WorkCentersListProductionRequestFilterItemValueThreeItemList(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_work_centers_list_production_request_filter_item_value_three_item_list(
        self,
    ) -> Option<Vec<WorkCentersListProductionRequestFilterItemValueThreeItem>> {
        match self {
            Self::WorkCentersListProductionRequestFilterItemValueThreeItemList(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}
