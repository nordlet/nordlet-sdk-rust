pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationReportResponseStatementsBalanceSheetDetail {
    #[serde(rename = "nonCurrentAssets")]
    #[serde(default)]
    pub non_current_assets:
        PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssets,
    #[serde(rename = "currentAssets")]
    #[serde(default)]
    pub current_assets: PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssets,
    #[serde(default)]
    pub equity: PostV1ConsolidationReportResponseStatementsBalanceSheetDetailEquity,
    #[serde(default)]
    pub liabilities: PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilities,
}

impl PostV1ConsolidationReportResponseStatementsBalanceSheetDetail {
    pub fn builder() -> PostV1ConsolidationReportResponseStatementsBalanceSheetDetailBuilder {
        <PostV1ConsolidationReportResponseStatementsBalanceSheetDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationReportResponseStatementsBalanceSheetDetailBuilder {
    non_current_assets:
        Option<PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssets>,
    current_assets:
        Option<PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssets>,
    equity: Option<PostV1ConsolidationReportResponseStatementsBalanceSheetDetailEquity>,
    liabilities: Option<PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilities>,
}

impl PostV1ConsolidationReportResponseStatementsBalanceSheetDetailBuilder {
    pub fn non_current_assets(
        mut self,
        value: PostV1ConsolidationReportResponseStatementsBalanceSheetDetailNonCurrentAssets,
    ) -> Self {
        self.non_current_assets = Some(value);
        self
    }

    pub fn current_assets(
        mut self,
        value: PostV1ConsolidationReportResponseStatementsBalanceSheetDetailCurrentAssets,
    ) -> Self {
        self.current_assets = Some(value);
        self
    }

    pub fn equity(
        mut self,
        value: PostV1ConsolidationReportResponseStatementsBalanceSheetDetailEquity,
    ) -> Self {
        self.equity = Some(value);
        self
    }

    pub fn liabilities(
        mut self,
        value: PostV1ConsolidationReportResponseStatementsBalanceSheetDetailLiabilities,
    ) -> Self {
        self.liabilities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationReportResponseStatementsBalanceSheetDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`non_current_assets`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailBuilder::non_current_assets)
    /// - [`current_assets`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailBuilder::current_assets)
    /// - [`equity`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailBuilder::equity)
    /// - [`liabilities`](PostV1ConsolidationReportResponseStatementsBalanceSheetDetailBuilder::liabilities)
    pub fn build(
        self,
    ) -> Result<PostV1ConsolidationReportResponseStatementsBalanceSheetDetail, BuildError> {
        Ok(
            PostV1ConsolidationReportResponseStatementsBalanceSheetDetail {
                non_current_assets: self
                    .non_current_assets
                    .ok_or_else(|| BuildError::missing_field("non_current_assets"))?,
                current_assets: self
                    .current_assets
                    .ok_or_else(|| BuildError::missing_field("current_assets"))?,
                equity: self
                    .equity
                    .ok_or_else(|| BuildError::missing_field("equity"))?,
                liabilities: self
                    .liabilities
                    .ok_or_else(|| BuildError::missing_field("liabilities"))?,
            },
        )
    }
}
