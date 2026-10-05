pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseStatementsBalanceSheetDetailLiabilities {
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

impl ReportConsolidationResponseStatementsBalanceSheetDetailLiabilities {
    pub fn builder() -> ReportConsolidationResponseStatementsBalanceSheetDetailLiabilitiesBuilder {
        <ReportConsolidationResponseStatementsBalanceSheetDetailLiabilitiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseStatementsBalanceSheetDetailLiabilitiesBuilder {
    non_current: Option<String>,
    current: Option<String>,
    other: Option<String>,
    total: Option<String>,
}

impl ReportConsolidationResponseStatementsBalanceSheetDetailLiabilitiesBuilder {
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

    /// Consumes the builder and constructs a [`ReportConsolidationResponseStatementsBalanceSheetDetailLiabilities`].
    /// This method will fail if any of the following fields are not set:
    /// - [`non_current`](ReportConsolidationResponseStatementsBalanceSheetDetailLiabilitiesBuilder::non_current)
    /// - [`current`](ReportConsolidationResponseStatementsBalanceSheetDetailLiabilitiesBuilder::current)
    /// - [`other`](ReportConsolidationResponseStatementsBalanceSheetDetailLiabilitiesBuilder::other)
    /// - [`total`](ReportConsolidationResponseStatementsBalanceSheetDetailLiabilitiesBuilder::total)
    pub fn build(
        self,
    ) -> Result<ReportConsolidationResponseStatementsBalanceSheetDetailLiabilities, BuildError>
    {
        Ok(
            ReportConsolidationResponseStatementsBalanceSheetDetailLiabilities {
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
