pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportConsolidationResponseStatementsBalanceSheetDetail {
    #[serde(rename = "nonCurrentAssets")]
    #[serde(default)]
    pub non_current_assets: ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssets,
    #[serde(rename = "currentAssets")]
    #[serde(default)]
    pub current_assets: ReportConsolidationResponseStatementsBalanceSheetDetailCurrentAssets,
    #[serde(default)]
    pub equity: ReportConsolidationResponseStatementsBalanceSheetDetailEquity,
    #[serde(default)]
    pub liabilities: ReportConsolidationResponseStatementsBalanceSheetDetailLiabilities,
}

impl ReportConsolidationResponseStatementsBalanceSheetDetail {
    pub fn builder() -> ReportConsolidationResponseStatementsBalanceSheetDetailBuilder {
        <ReportConsolidationResponseStatementsBalanceSheetDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportConsolidationResponseStatementsBalanceSheetDetailBuilder {
    non_current_assets:
        Option<ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssets>,
    current_assets: Option<ReportConsolidationResponseStatementsBalanceSheetDetailCurrentAssets>,
    equity: Option<ReportConsolidationResponseStatementsBalanceSheetDetailEquity>,
    liabilities: Option<ReportConsolidationResponseStatementsBalanceSheetDetailLiabilities>,
}

impl ReportConsolidationResponseStatementsBalanceSheetDetailBuilder {
    pub fn non_current_assets(
        mut self,
        value: ReportConsolidationResponseStatementsBalanceSheetDetailNonCurrentAssets,
    ) -> Self {
        self.non_current_assets = Some(value);
        self
    }

    pub fn current_assets(
        mut self,
        value: ReportConsolidationResponseStatementsBalanceSheetDetailCurrentAssets,
    ) -> Self {
        self.current_assets = Some(value);
        self
    }

    pub fn equity(
        mut self,
        value: ReportConsolidationResponseStatementsBalanceSheetDetailEquity,
    ) -> Self {
        self.equity = Some(value);
        self
    }

    pub fn liabilities(
        mut self,
        value: ReportConsolidationResponseStatementsBalanceSheetDetailLiabilities,
    ) -> Self {
        self.liabilities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportConsolidationResponseStatementsBalanceSheetDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`non_current_assets`](ReportConsolidationResponseStatementsBalanceSheetDetailBuilder::non_current_assets)
    /// - [`current_assets`](ReportConsolidationResponseStatementsBalanceSheetDetailBuilder::current_assets)
    /// - [`equity`](ReportConsolidationResponseStatementsBalanceSheetDetailBuilder::equity)
    /// - [`liabilities`](ReportConsolidationResponseStatementsBalanceSheetDetailBuilder::liabilities)
    pub fn build(
        self,
    ) -> Result<ReportConsolidationResponseStatementsBalanceSheetDetail, BuildError> {
        Ok(ReportConsolidationResponseStatementsBalanceSheetDetail {
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
        })
    }
}
