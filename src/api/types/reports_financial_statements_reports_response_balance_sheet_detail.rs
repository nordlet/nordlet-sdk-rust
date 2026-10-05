pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FinancialStatementsReportsResponseBalanceSheetDetail {
    #[serde(rename = "nonCurrentAssets")]
    #[serde(default)]
    pub non_current_assets: FinancialStatementsReportsResponseBalanceSheetDetailNonCurrentAssets,
    #[serde(rename = "currentAssets")]
    #[serde(default)]
    pub current_assets: FinancialStatementsReportsResponseBalanceSheetDetailCurrentAssets,
    #[serde(default)]
    pub equity: FinancialStatementsReportsResponseBalanceSheetDetailEquity,
    #[serde(default)]
    pub liabilities: FinancialStatementsReportsResponseBalanceSheetDetailLiabilities,
}

impl FinancialStatementsReportsResponseBalanceSheetDetail {
    pub fn builder() -> FinancialStatementsReportsResponseBalanceSheetDetailBuilder {
        <FinancialStatementsReportsResponseBalanceSheetDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancialStatementsReportsResponseBalanceSheetDetailBuilder {
    non_current_assets:
        Option<FinancialStatementsReportsResponseBalanceSheetDetailNonCurrentAssets>,
    current_assets: Option<FinancialStatementsReportsResponseBalanceSheetDetailCurrentAssets>,
    equity: Option<FinancialStatementsReportsResponseBalanceSheetDetailEquity>,
    liabilities: Option<FinancialStatementsReportsResponseBalanceSheetDetailLiabilities>,
}

impl FinancialStatementsReportsResponseBalanceSheetDetailBuilder {
    pub fn non_current_assets(
        mut self,
        value: FinancialStatementsReportsResponseBalanceSheetDetailNonCurrentAssets,
    ) -> Self {
        self.non_current_assets = Some(value);
        self
    }

    pub fn current_assets(
        mut self,
        value: FinancialStatementsReportsResponseBalanceSheetDetailCurrentAssets,
    ) -> Self {
        self.current_assets = Some(value);
        self
    }

    pub fn equity(
        mut self,
        value: FinancialStatementsReportsResponseBalanceSheetDetailEquity,
    ) -> Self {
        self.equity = Some(value);
        self
    }

    pub fn liabilities(
        mut self,
        value: FinancialStatementsReportsResponseBalanceSheetDetailLiabilities,
    ) -> Self {
        self.liabilities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FinancialStatementsReportsResponseBalanceSheetDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`non_current_assets`](FinancialStatementsReportsResponseBalanceSheetDetailBuilder::non_current_assets)
    /// - [`current_assets`](FinancialStatementsReportsResponseBalanceSheetDetailBuilder::current_assets)
    /// - [`equity`](FinancialStatementsReportsResponseBalanceSheetDetailBuilder::equity)
    /// - [`liabilities`](FinancialStatementsReportsResponseBalanceSheetDetailBuilder::liabilities)
    pub fn build(self) -> Result<FinancialStatementsReportsResponseBalanceSheetDetail, BuildError> {
        Ok(FinancialStatementsReportsResponseBalanceSheetDetail {
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
