pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItem {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub quantity: String,
    #[serde(default)]
    pub unit: String,
    #[serde(rename = "unitPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<String>,
    #[serde(rename = "taxableAmount")]
    #[serde(default)]
    pub taxable_amount: String,
    #[serde(rename = "vatRatePercent")]
    #[serde(default)]
    pub vat_rate_percent: String,
    #[serde(rename = "vatAmount")]
    #[serde(default)]
    pub vat_amount: String,
}

impl EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItem {
    pub fn builder() -> EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItemBuilder {
        <EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItemBuilder {
    description: Option<String>,
    quantity: Option<String>,
    unit: Option<String>,
    unit_price: Option<String>,
    taxable_amount: Option<String>,
    vat_rate_percent: Option<String>,
    vat_amount: Option<String>,
}

impl EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItemBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: impl Into<String>) -> Self {
        self.quantity = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn unit_price(mut self, value: impl Into<String>) -> Self {
        self.unit_price = Some(value.into());
        self
    }

    pub fn taxable_amount(mut self, value: impl Into<String>) -> Self {
        self.taxable_amount = Some(value.into());
        self
    }

    pub fn vat_rate_percent(mut self, value: impl Into<String>) -> Self {
        self.vat_rate_percent = Some(value.into());
        self
    }

    pub fn vat_amount(mut self, value: impl Into<String>) -> Self {
        self.vat_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItemBuilder::description)
    /// - [`quantity`](EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItemBuilder::quantity)
    /// - [`unit`](EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItemBuilder::unit)
    /// - [`taxable_amount`](EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItemBuilder::taxable_amount)
    /// - [`vat_rate_percent`](EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItemBuilder::vat_rate_percent)
    /// - [`vat_amount`](EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItemBuilder::vat_amount)
    pub fn build(
        self,
    ) -> Result<EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItem, BuildError>
    {
        Ok(
            EuDigitalReportingListDeclarationsResponseTransactionsItemLinesItem {
                description: self
                    .description
                    .ok_or_else(|| BuildError::missing_field("description"))?,
                quantity: self
                    .quantity
                    .ok_or_else(|| BuildError::missing_field("quantity"))?,
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
                unit_price: self.unit_price,
                taxable_amount: self
                    .taxable_amount
                    .ok_or_else(|| BuildError::missing_field("taxable_amount"))?,
                vat_rate_percent: self
                    .vat_rate_percent
                    .ok_or_else(|| BuildError::missing_field("vat_rate_percent"))?,
                vat_amount: self
                    .vat_amount
                    .ok_or_else(|| BuildError::missing_field("vat_amount"))?,
            },
        )
    }
}
