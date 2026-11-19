pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Us941GetDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub quarter: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposits: Option<String>,
}

impl Us941GetDeclarationsRequest {
    pub fn builder() -> Us941GetDeclarationsRequestBuilder {
        <Us941GetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Us941GetDeclarationsRequestBuilder {
    year: Option<i64>,
    quarter: Option<i64>,
    deposits: Option<String>,
}

impl Us941GetDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn quarter(mut self, value: i64) -> Self {
        self.quarter = Some(value);
        self
    }

    pub fn deposits(mut self, value: impl Into<String>) -> Self {
        self.deposits = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Us941GetDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](Us941GetDeclarationsRequestBuilder::year)
    /// - [`quarter`](Us941GetDeclarationsRequestBuilder::quarter)
    pub fn build(self) -> Result<Us941GetDeclarationsRequest, BuildError> {
        Ok(Us941GetDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            quarter: self
                .quarter
                .ok_or_else(|| BuildError::missing_field("quarter"))?,
            deposits: self.deposits,
        })
    }
}
