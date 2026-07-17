pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponseNonControllingInterest {
    #[serde(default)]
    pub equity: String,
    #[serde(default)]
    pub result: String,
}

impl PostV1ConsolidationReportResponseNonControllingInterest {
    pub fn builder() -> PostV1ConsolidationReportResponseNonControllingInterestBuilder {
        <PostV1ConsolidationReportResponseNonControllingInterestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseNonControllingInterestBuilder {
    equity: Option<String>,
    result: Option<String>,
}

impl PostV1ConsolidationReportResponseNonControllingInterestBuilder {
    pub fn equity(mut self, value: impl Into<String>) -> Self {
        self.equity = Some(value.into());
        self
    }

    pub fn result(mut self, value: impl Into<String>) -> Self {
        self.result = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponseNonControllingInterest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`equity`](PostV1ConsolidationReportResponseNonControllingInterestBuilder::equity)
    /// - [`result`](PostV1ConsolidationReportResponseNonControllingInterestBuilder::result)
    pub fn build(
        self,
    ) -> Result<PostV1ConsolidationReportResponseNonControllingInterest, BuildError> {
        Ok(PostV1ConsolidationReportResponseNonControllingInterest {
            equity: self
                .equity
                .ok_or_else(|| BuildError::missing_field("equity"))?,
            result: self
                .result
                .ok_or_else(|| BuildError::missing_field("result"))?,
        })
    }
}
