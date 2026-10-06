pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DebtRemindersPreviewPartnersResponseRowsItemTotalsItem {
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "totalDue")]
    #[serde(default)]
    pub total_due: String,
    #[serde(rename = "interestDue")]
    #[serde(default)]
    pub interest_due: String,
}

impl DebtRemindersPreviewPartnersResponseRowsItemTotalsItem {
    pub fn builder() -> DebtRemindersPreviewPartnersResponseRowsItemTotalsItemBuilder {
        <DebtRemindersPreviewPartnersResponseRowsItemTotalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtRemindersPreviewPartnersResponseRowsItemTotalsItemBuilder {
    currency: Option<String>,
    total_due: Option<String>,
    interest_due: Option<String>,
}

impl DebtRemindersPreviewPartnersResponseRowsItemTotalsItemBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn total_due(mut self, value: impl Into<String>) -> Self {
        self.total_due = Some(value.into());
        self
    }

    pub fn interest_due(mut self, value: impl Into<String>) -> Self {
        self.interest_due = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DebtRemindersPreviewPartnersResponseRowsItemTotalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](DebtRemindersPreviewPartnersResponseRowsItemTotalsItemBuilder::currency)
    /// - [`total_due`](DebtRemindersPreviewPartnersResponseRowsItemTotalsItemBuilder::total_due)
    /// - [`interest_due`](DebtRemindersPreviewPartnersResponseRowsItemTotalsItemBuilder::interest_due)
    pub fn build(
        self,
    ) -> Result<DebtRemindersPreviewPartnersResponseRowsItemTotalsItem, BuildError> {
        Ok(DebtRemindersPreviewPartnersResponseRowsItemTotalsItem {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            total_due: self
                .total_due
                .ok_or_else(|| BuildError::missing_field("total_due"))?,
            interest_due: self
                .interest_due
                .ok_or_else(|| BuildError::missing_field("interest_due"))?,
        })
    }
}
