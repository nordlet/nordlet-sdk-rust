pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponseEquityMethod {
    #[serde(rename = "investmentsInAssociates")]
    #[serde(default)]
    pub investments_in_associates: String,
    #[serde(rename = "shareOfAssociatesResult")]
    #[serde(default)]
    pub share_of_associates_result: String,
}

impl PostV1ConsolidationReportResponseEquityMethod {
    pub fn builder() -> PostV1ConsolidationReportResponseEquityMethodBuilder {
        <PostV1ConsolidationReportResponseEquityMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseEquityMethodBuilder {
    investments_in_associates: Option<String>,
    share_of_associates_result: Option<String>,
}

impl PostV1ConsolidationReportResponseEquityMethodBuilder {
    pub fn investments_in_associates(mut self, value: impl Into<String>) -> Self {
        self.investments_in_associates = Some(value.into());
        self
    }

    pub fn share_of_associates_result(mut self, value: impl Into<String>) -> Self {
        self.share_of_associates_result = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponseEquityMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`investments_in_associates`](PostV1ConsolidationReportResponseEquityMethodBuilder::investments_in_associates)
    /// - [`share_of_associates_result`](PostV1ConsolidationReportResponseEquityMethodBuilder::share_of_associates_result)
    pub fn build(self) -> Result<PostV1ConsolidationReportResponseEquityMethod, BuildError> {
        Ok(PostV1ConsolidationReportResponseEquityMethod {
            investments_in_associates: self
                .investments_in_associates
                .ok_or_else(|| BuildError::missing_field("investments_in_associates"))?,
            share_of_associates_result: self
                .share_of_associates_result
                .ok_or_else(|| BuildError::missing_field("share_of_associates_result"))?,
        })
    }
}
