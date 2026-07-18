pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuSmeCrossBorderReportComputeRequest {
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub quarter: i64,
}

impl PostV1DeclarationsEuSmeCrossBorderReportComputeRequest {
    pub fn builder() -> PostV1DeclarationsEuSmeCrossBorderReportComputeRequestBuilder {
        <PostV1DeclarationsEuSmeCrossBorderReportComputeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuSmeCrossBorderReportComputeRequestBuilder {
    year: Option<i64>,
    quarter: Option<i64>,
}

impl PostV1DeclarationsEuSmeCrossBorderReportComputeRequestBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn quarter(mut self, value: i64) -> Self {
        self.quarter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuSmeCrossBorderReportComputeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsEuSmeCrossBorderReportComputeRequestBuilder::year)
    /// - [`quarter`](PostV1DeclarationsEuSmeCrossBorderReportComputeRequestBuilder::quarter)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsEuSmeCrossBorderReportComputeRequest, BuildError> {
        Ok(PostV1DeclarationsEuSmeCrossBorderReportComputeRequest {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            quarter: self
                .quarter
                .ok_or_else(|| BuildError::missing_field("quarter"))?,
        })
    }
}
