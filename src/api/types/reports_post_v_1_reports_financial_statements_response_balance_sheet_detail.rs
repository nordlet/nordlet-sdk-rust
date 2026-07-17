pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsFinancialStatementsResponseBalanceSheetDetail {
    #[serde(rename = "nonCurrentAssets")]
    #[serde(default)]
    pub non_current_assets:
        PostV1ReportsFinancialStatementsResponseBalanceSheetDetailNonCurrentAssets,
    #[serde(rename = "currentAssets")]
    #[serde(default)]
    pub current_assets: PostV1ReportsFinancialStatementsResponseBalanceSheetDetailCurrentAssets,
    #[serde(default)]
    pub equity: PostV1ReportsFinancialStatementsResponseBalanceSheetDetailEquity,
    #[serde(default)]
    pub liabilities: PostV1ReportsFinancialStatementsResponseBalanceSheetDetailLiabilities,
}

impl PostV1ReportsFinancialStatementsResponseBalanceSheetDetail {
    pub fn builder() -> PostV1ReportsFinancialStatementsResponseBalanceSheetDetailBuilder {
        <PostV1ReportsFinancialStatementsResponseBalanceSheetDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsFinancialStatementsResponseBalanceSheetDetailBuilder {
    non_current_assets:
        Option<PostV1ReportsFinancialStatementsResponseBalanceSheetDetailNonCurrentAssets>,
    current_assets: Option<PostV1ReportsFinancialStatementsResponseBalanceSheetDetailCurrentAssets>,
    equity: Option<PostV1ReportsFinancialStatementsResponseBalanceSheetDetailEquity>,
    liabilities: Option<PostV1ReportsFinancialStatementsResponseBalanceSheetDetailLiabilities>,
}

impl PostV1ReportsFinancialStatementsResponseBalanceSheetDetailBuilder {
    pub fn non_current_assets(
        mut self,
        value: PostV1ReportsFinancialStatementsResponseBalanceSheetDetailNonCurrentAssets,
    ) -> Self {
        self.non_current_assets = Some(value);
        self
    }

    pub fn current_assets(
        mut self,
        value: PostV1ReportsFinancialStatementsResponseBalanceSheetDetailCurrentAssets,
    ) -> Self {
        self.current_assets = Some(value);
        self
    }

    pub fn equity(
        mut self,
        value: PostV1ReportsFinancialStatementsResponseBalanceSheetDetailEquity,
    ) -> Self {
        self.equity = Some(value);
        self
    }

    pub fn liabilities(
        mut self,
        value: PostV1ReportsFinancialStatementsResponseBalanceSheetDetailLiabilities,
    ) -> Self {
        self.liabilities = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsFinancialStatementsResponseBalanceSheetDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`non_current_assets`](PostV1ReportsFinancialStatementsResponseBalanceSheetDetailBuilder::non_current_assets)
    /// - [`current_assets`](PostV1ReportsFinancialStatementsResponseBalanceSheetDetailBuilder::current_assets)
    /// - [`equity`](PostV1ReportsFinancialStatementsResponseBalanceSheetDetailBuilder::equity)
    /// - [`liabilities`](PostV1ReportsFinancialStatementsResponseBalanceSheetDetailBuilder::liabilities)
    pub fn build(
        self,
    ) -> Result<PostV1ReportsFinancialStatementsResponseBalanceSheetDetail, BuildError> {
        Ok(PostV1ReportsFinancialStatementsResponseBalanceSheetDetail {
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
