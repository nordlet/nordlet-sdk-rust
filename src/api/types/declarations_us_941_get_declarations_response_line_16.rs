pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Us941GetDeclarationsResponseLine16 {
    #[serde(default)]
    pub months: Vec<Us941GetDeclarationsResponseLine16MonthsItem>,
    #[serde(default)]
    pub total: String,
}

impl Us941GetDeclarationsResponseLine16 {
    pub fn builder() -> Us941GetDeclarationsResponseLine16Builder {
        <Us941GetDeclarationsResponseLine16Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Us941GetDeclarationsResponseLine16Builder {
    months: Option<Vec<Us941GetDeclarationsResponseLine16MonthsItem>>,
    total: Option<String>,
}

impl Us941GetDeclarationsResponseLine16Builder {
    pub fn months(mut self, value: Vec<Us941GetDeclarationsResponseLine16MonthsItem>) -> Self {
        self.months = Some(value);
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Us941GetDeclarationsResponseLine16`].
    /// This method will fail if any of the following fields are not set:
    /// - [`months`](Us941GetDeclarationsResponseLine16Builder::months)
    /// - [`total`](Us941GetDeclarationsResponseLine16Builder::total)
    pub fn build(self) -> Result<Us941GetDeclarationsResponseLine16, BuildError> {
        Ok(Us941GetDeclarationsResponseLine16 {
            months: self
                .months
                .ok_or_else(|| BuildError::missing_field("months"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
