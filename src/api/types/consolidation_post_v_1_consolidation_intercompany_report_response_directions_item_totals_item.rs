pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItem {
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "salesGross")]
    #[serde(default)]
    pub sales_gross: String,
    #[serde(rename = "purchasesGross")]
    #[serde(default)]
    pub purchases_gross: String,
    #[serde(rename = "grossDifference")]
    #[serde(default)]
    pub gross_difference: String,
    #[serde(rename = "openReceivable")]
    #[serde(default)]
    pub open_receivable: String,
    #[serde(rename = "openPayable")]
    #[serde(default)]
    pub open_payable: String,
    #[serde(rename = "openDifference")]
    #[serde(default)]
    pub open_difference: String,
}

impl PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItem {
    pub fn builder() -> PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItemBuilder
    {
        <PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItemBuilder {
    currency: Option<String>,
    sales_gross: Option<String>,
    purchases_gross: Option<String>,
    gross_difference: Option<String>,
    open_receivable: Option<String>,
    open_payable: Option<String>,
    open_difference: Option<String>,
}

impl PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItemBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn sales_gross(mut self, value: impl Into<String>) -> Self {
        self.sales_gross = Some(value.into());
        self
    }

    pub fn purchases_gross(mut self, value: impl Into<String>) -> Self {
        self.purchases_gross = Some(value.into());
        self
    }

    pub fn gross_difference(mut self, value: impl Into<String>) -> Self {
        self.gross_difference = Some(value.into());
        self
    }

    pub fn open_receivable(mut self, value: impl Into<String>) -> Self {
        self.open_receivable = Some(value.into());
        self
    }

    pub fn open_payable(mut self, value: impl Into<String>) -> Self {
        self.open_payable = Some(value.into());
        self
    }

    pub fn open_difference(mut self, value: impl Into<String>) -> Self {
        self.open_difference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItemBuilder::currency)
    /// - [`sales_gross`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItemBuilder::sales_gross)
    /// - [`purchases_gross`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItemBuilder::purchases_gross)
    /// - [`gross_difference`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItemBuilder::gross_difference)
    /// - [`open_receivable`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItemBuilder::open_receivable)
    /// - [`open_payable`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItemBuilder::open_payable)
    /// - [`open_difference`](PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItemBuilder::open_difference)
    pub fn build(
        self,
    ) -> Result<PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItem, BuildError>
    {
        Ok(
            PostV1ConsolidationIntercompanyReportResponseDirectionsItemTotalsItem {
                currency: self
                    .currency
                    .ok_or_else(|| BuildError::missing_field("currency"))?,
                sales_gross: self
                    .sales_gross
                    .ok_or_else(|| BuildError::missing_field("sales_gross"))?,
                purchases_gross: self
                    .purchases_gross
                    .ok_or_else(|| BuildError::missing_field("purchases_gross"))?,
                gross_difference: self
                    .gross_difference
                    .ok_or_else(|| BuildError::missing_field("gross_difference"))?,
                open_receivable: self
                    .open_receivable
                    .ok_or_else(|| BuildError::missing_field("open_receivable"))?,
                open_payable: self
                    .open_payable
                    .ok_or_else(|| BuildError::missing_field("open_payable"))?,
                open_difference: self
                    .open_difference
                    .ok_or_else(|| BuildError::missing_field("open_difference"))?,
            },
        )
    }
}
