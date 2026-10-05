pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseEquityMethod {
    #[serde(rename = "investmentsInAssociates")]
    #[serde(default)]
    pub investments_in_associates: String,
    #[serde(rename = "shareOfAssociatesResult")]
    #[serde(default)]
    pub share_of_associates_result: String,
}

impl ReportConsolidationResponseEquityMethod {
    pub fn builder() -> ReportConsolidationResponseEquityMethodBuilder {
        <ReportConsolidationResponseEquityMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseEquityMethodBuilder {
    investments_in_associates: Option<String>,
    share_of_associates_result: Option<String>,
}

impl ReportConsolidationResponseEquityMethodBuilder {
    pub fn investments_in_associates(mut self, value: impl Into<String>) -> Self {
        self.investments_in_associates = Some(value.into());
        self
    }

    pub fn share_of_associates_result(mut self, value: impl Into<String>) -> Self {
        self.share_of_associates_result = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReportConsolidationResponseEquityMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`investments_in_associates`](ReportConsolidationResponseEquityMethodBuilder::investments_in_associates)
    /// - [`share_of_associates_result`](ReportConsolidationResponseEquityMethodBuilder::share_of_associates_result)
    pub fn build(self) -> Result<ReportConsolidationResponseEquityMethod, BuildError> {
        Ok(ReportConsolidationResponseEquityMethod {
            investments_in_associates: self
                .investments_in_associates
                .ok_or_else(|| BuildError::missing_field("investments_in_associates"))?,
            share_of_associates_result: self
                .share_of_associates_result
                .ok_or_else(|| BuildError::missing_field("share_of_associates_result"))?,
        })
    }
}
