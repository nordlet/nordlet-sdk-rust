pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssets {
    #[serde(default)]
    pub intangible: String,
    #[serde(default)]
    pub tangible: String,
    #[serde(default)]
    pub financial: String,
    #[serde(default)]
    pub other: String,
    #[serde(default)]
    pub total: String,
}

impl ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssets {
    pub fn builder(
    ) -> ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder {
        <ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder {
    intangible: Option<String>,
    tangible: Option<String>,
    financial: Option<String>,
    other: Option<String>,
    total: Option<String>,
}

impl ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder {
    pub fn intangible(mut self, value: impl Into<String>) -> Self {
        self.intangible = Some(value.into());
        self
    }

    pub fn tangible(mut self, value: impl Into<String>) -> Self {
        self.tangible = Some(value.into());
        self
    }

    pub fn financial(mut self, value: impl Into<String>) -> Self {
        self.financial = Some(value.into());
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

    /// Consumes the builder and constructs a [`ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssets`].
    /// This method will fail if any of the following fields are not set:
    /// - [`intangible`](ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder::intangible)
    /// - [`tangible`](ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder::tangible)
    /// - [`financial`](ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder::financial)
    /// - [`other`](ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder::other)
    /// - [`total`](ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder::total)
    pub fn build(
        self,
    ) -> Result<ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssets, BuildError>
    {
        Ok(
            ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssets {
                intangible: self
                    .intangible
                    .ok_or_else(|| BuildError::missing_field("intangible"))?,
                tangible: self
                    .tangible
                    .ok_or_else(|| BuildError::missing_field("tangible"))?,
                financial: self
                    .financial
                    .ok_or_else(|| BuildError::missing_field("financial"))?,
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
