pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuOssComputeDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub quarter: i64,
}

impl EuOssComputeDeclarationsRequest {
    pub fn builder() -> EuOssComputeDeclarationsRequestBuilder {
        <EuOssComputeDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuOssComputeDeclarationsRequestBuilder {
    year: Option<i64>,
    quarter: Option<i64>,
}

impl EuOssComputeDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn quarter(mut self, value: i64) -> Self {
        self.quarter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuOssComputeDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](EuOssComputeDeclarationsRequestBuilder::year)
    /// - [`quarter`](EuOssComputeDeclarationsRequestBuilder::quarter)
    pub fn build(self) -> Result<EuOssComputeDeclarationsRequest, BuildError> {
        Ok(EuOssComputeDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            quarter: self
                .quarter
                .ok_or_else(|| BuildError::missing_field("quarter"))?,
        })
    }
}
