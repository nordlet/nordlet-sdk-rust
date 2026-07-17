pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssets {
    #[serde(default)]
    pub inventories: String,
    #[serde(default)]
    pub receivables: String,
    #[serde(rename = "otherCurrent")]
    #[serde(default)]
    pub other_current: String,
    #[serde(default)]
    pub cash: String,
    #[serde(default)]
    pub total: String,
}

impl PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssets {
    pub fn builder(
    ) -> PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssetsBuilder {
        <PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssetsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssetsBuilder {
    inventories: Option<String>,
    receivables: Option<String>,
    other_current: Option<String>,
    cash: Option<String>,
    total: Option<String>,
}

impl PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssetsBuilder {
    pub fn inventories(mut self, value: impl Into<String>) -> Self {
        self.inventories = Some(value.into());
        self
    }

    pub fn receivables(mut self, value: impl Into<String>) -> Self {
        self.receivables = Some(value.into());
        self
    }

    pub fn other_current(mut self, value: impl Into<String>) -> Self {
        self.other_current = Some(value.into());
        self
    }

    pub fn cash(mut self, value: impl Into<String>) -> Self {
        self.cash = Some(value.into());
        self
    }

    pub fn total(mut self, value: impl Into<String>) -> Self {
        self.total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssets`].
    /// This method will fail if any of the following fields are not set:
    /// - [`inventories`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssetsBuilder::inventories)
    /// - [`receivables`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssetsBuilder::receivables)
    /// - [`other_current`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssetsBuilder::other_current)
    /// - [`cash`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssetsBuilder::cash)
    /// - [`total`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssetsBuilder::total)
    pub fn build(
        self,
    ) -> Result<
        PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssets,
        BuildError,
    > {
        Ok(
            PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssets {
                inventories: self
                    .inventories
                    .ok_or_else(|| BuildError::missing_field("inventories"))?,
                receivables: self
                    .receivables
                    .ok_or_else(|| BuildError::missing_field("receivables"))?,
                other_current: self
                    .other_current
                    .ok_or_else(|| BuildError::missing_field("other_current"))?,
                cash: self.cash.ok_or_else(|| BuildError::missing_field("cash"))?,
                total: self
                    .total
                    .ok_or_else(|| BuildError::missing_field("total"))?,
            },
        )
    }
}
