pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlZusDraComputeDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
}

impl PlZusDraComputeDeclarationsRequest {
    pub fn builder() -> PlZusDraComputeDeclarationsRequestBuilder {
        <PlZusDraComputeDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlZusDraComputeDeclarationsRequestBuilder {
    year: Option<i64>,
    month: Option<i64>,
}

impl PlZusDraComputeDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlZusDraComputeDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PlZusDraComputeDeclarationsRequestBuilder::year)
    /// - [`month`](PlZusDraComputeDeclarationsRequestBuilder::month)
    pub fn build(self) -> Result<PlZusDraComputeDeclarationsRequest, BuildError> {
        Ok(PlZusDraComputeDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
        })
    }
}
