pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssets {
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

impl PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssets {
    pub fn builder(
    ) -> PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder {
        <PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder {
    intangible: Option<String>,
    tangible: Option<String>,
    financial: Option<String>,
    other: Option<String>,
    total: Option<String>,
}

impl PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssets`].
    /// This method will fail if any of the following fields are not set:
    /// - [`intangible`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder::intangible)
    /// - [`tangible`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder::tangible)
    /// - [`financial`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder::financial)
    /// - [`other`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder::other)
    /// - [`total`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssetsBuilder::total)
    pub fn build(
        self,
    ) -> Result<
        PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssets,
        BuildError,
    > {
        Ok(
            PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssets {
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
