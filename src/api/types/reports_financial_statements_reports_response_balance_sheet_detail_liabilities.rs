pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FinancialStatementsReportsResponseBalanceSheetDetailLiabilities {
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

impl FinancialStatementsReportsResponseBalanceSheetDetailLiabilities {
    pub fn builder() -> FinancialStatementsReportsResponseBalanceSheetDetailLiabilitiesBuilder {
        <FinancialStatementsReportsResponseBalanceSheetDetailLiabilitiesBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancialStatementsReportsResponseBalanceSheetDetailLiabilitiesBuilder {
    non_current: Option<String>,
    current: Option<String>,
    other: Option<String>,
    total: Option<String>,
}

impl FinancialStatementsReportsResponseBalanceSheetDetailLiabilitiesBuilder {
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

    /// Consumes the builder and constructs a [`FinancialStatementsReportsResponseBalanceSheetDetailLiabilities`].
    /// This method will fail if any of the following fields are not set:
    /// - [`non_current`](FinancialStatementsReportsResponseBalanceSheetDetailLiabilitiesBuilder::non_current)
    /// - [`current`](FinancialStatementsReportsResponseBalanceSheetDetailLiabilitiesBuilder::current)
    /// - [`other`](FinancialStatementsReportsResponseBalanceSheetDetailLiabilitiesBuilder::other)
    /// - [`total`](FinancialStatementsReportsResponseBalanceSheetDetailLiabilitiesBuilder::total)
    pub fn build(
        self,
    ) -> Result<FinancialStatementsReportsResponseBalanceSheetDetailLiabilities, BuildError> {
        Ok(
            FinancialStatementsReportsResponseBalanceSheetDetailLiabilities {
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
