pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuSmeCrossBorderReportComputeDeclarationsRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub quarter: i64,
}

impl EuSmeCrossBorderReportComputeDeclarationsRequest {
    pub fn builder() -> EuSmeCrossBorderReportComputeDeclarationsRequestBuilder {
        <EuSmeCrossBorderReportComputeDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuSmeCrossBorderReportComputeDeclarationsRequestBuilder {
    year: Option<i64>,
    quarter: Option<i64>,
}

impl EuSmeCrossBorderReportComputeDeclarationsRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn quarter(mut self, value: i64) -> Self {
        self.quarter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EuSmeCrossBorderReportComputeDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](EuSmeCrossBorderReportComputeDeclarationsRequestBuilder::year)
    /// - [`quarter`](EuSmeCrossBorderReportComputeDeclarationsRequestBuilder::quarter)
    pub fn build(self) -> Result<EuSmeCrossBorderReportComputeDeclarationsRequest, BuildError> {
        Ok(EuSmeCrossBorderReportComputeDeclarationsRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            quarter: self
                .quarter
                .ok_or_else(|| BuildError::missing_field("quarter"))?,
        })
    }
}
