pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseNonControllingInterest {
    #[serde(default)]
    pub equity: String,
    #[serde(default)]
    pub result: String,
}

impl ReportConsolidationResponseNonControllingInterest {
    pub fn builder() -> ReportConsolidationResponseNonControllingInterestBuilder {
        <ReportConsolidationResponseNonControllingInterestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseNonControllingInterestBuilder {
    equity: Option<String>,
    result: Option<String>,
}

impl ReportConsolidationResponseNonControllingInterestBuilder {
    pub fn equity(mut self, value: impl Into<String>) -> Self {
        self.equity = Some(value.into());
        self
    }

    pub fn result(mut self, value: impl Into<String>) -> Self {
        self.result = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReportConsolidationResponseNonControllingInterest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`equity`](ReportConsolidationResponseNonControllingInterestBuilder::equity)
    /// - [`result`](ReportConsolidationResponseNonControllingInterestBuilder::result)
    pub fn build(self) -> Result<ReportConsolidationResponseNonControllingInterest, BuildError> {
        Ok(ReportConsolidationResponseNonControllingInterest {
            equity: self
                .equity
                .ok_or_else(|| BuildError::missing_field("equity"))?,
            result: self
                .result
                .ok_or_else(|| BuildError::missing_field("result"))?,
        })
    }
}
