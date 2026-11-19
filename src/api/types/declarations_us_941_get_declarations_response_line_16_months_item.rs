pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Us941GetDeclarationsResponseLine16MonthsItem {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub month: f64,
    #[serde(default)]
    pub liability: String,
}

impl Us941GetDeclarationsResponseLine16MonthsItem {
    pub fn builder() -> Us941GetDeclarationsResponseLine16MonthsItemBuilder {
        <Us941GetDeclarationsResponseLine16MonthsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Us941GetDeclarationsResponseLine16MonthsItemBuilder {
    month: Option<f64>,
    liability: Option<String>,
}

impl Us941GetDeclarationsResponseLine16MonthsItemBuilder {
    pub fn month(mut self, value: f64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn liability(mut self, value: impl Into<String>) -> Self {
        self.liability = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Us941GetDeclarationsResponseLine16MonthsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`month`](Us941GetDeclarationsResponseLine16MonthsItemBuilder::month)
    /// - [`liability`](Us941GetDeclarationsResponseLine16MonthsItemBuilder::liability)
    pub fn build(self) -> Result<Us941GetDeclarationsResponseLine16MonthsItem, BuildError> {
        Ok(Us941GetDeclarationsResponseLine16MonthsItem {
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            liability: self
                .liability
                .ok_or_else(|| BuildError::missing_field("liability"))?,
        })
    }
}
