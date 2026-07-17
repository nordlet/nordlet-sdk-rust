pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilities {
    #[serde(rename = "nonCurrent")]
    #[serde(default)]
    pub non_current: String,
    #[serde(default)]
    pub current: String,
    #[serde(default)]
    pub other: String,
    #[serde(default)]
    pub total: String,
}

impl PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilities {
    pub fn builder(
    ) -> PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilitiesBuilder {
        <PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilitiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilitiesBuilder {
    non_current: Option<String>,
    current: Option<String>,
    other: Option<String>,
    total: Option<String>,
}

impl PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilitiesBuilder {
    pub fn non_current(mut self, value: impl Into<String>) -> Self {
        self.non_current = Some(value.into());
        self
    }

    pub fn current(mut self, value: impl Into<String>) -> Self {
        self.current = Some(value.into());
        self
    }

    pub fn other(mut self, value: impl Into<String>) -> Self {
        self.other = Some(value.into());
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilities`].
    /// This method will fail if any of the following fields are not set:
    /// - [`non_current`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilitiesBuilder::non_current)
    /// - [`current`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilitiesBuilder::current)
    /// - [`other`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilitiesBuilder::other)
    /// - [`total`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilitiesBuilder::total)
    pub fn build(
        self,
    ) -> Result<PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilities, BuildError>
    {
        Ok(
            PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilities {
                non_current: self
                    .non_current
                    .ok_or_else(|| BuildError::missing_field("non_current"))?,
                current: self
                    .current
                    .ok_or_else(|| BuildError::missing_field("current"))?,
                other: self
                    .other
                    .ok_or_else(|| BuildError::missing_field("other"))?,
                total: self
                    .total
                    .ok_or_else(|| BuildError::missing_field("total"))?,
            },
        )
    }
}
