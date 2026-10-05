pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PlVatUeGenerateDeclarationsResponseTotalsItem {
    pub section: PlVatUeGenerateDeclarationsResponseTotalsItemSection,
    #[serde(default)]
    pub counterparties: i64,
    #[serde(default)]
    pub amount: String,
}

impl PlVatUeGenerateDeclarationsResponseTotalsItem {
    pub fn builder() -> PlVatUeGenerateDeclarationsResponseTotalsItemBuilder {
        <PlVatUeGenerateDeclarationsResponseTotalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlVatUeGenerateDeclarationsResponseTotalsItemBuilder {
    section: Option<PlVatUeGenerateDeclarationsResponseTotalsItemSection>,
    counterparties: Option<i64>,
    amount: Option<String>,
}

impl PlVatUeGenerateDeclarationsResponseTotalsItemBuilder {
    pub fn section(mut self, value: PlVatUeGenerateDeclarationsResponseTotalsItemSection) -> Self {
        self.section = Some(value);
        self
    }

    pub fn counterparties(mut self, value: i64) -> Self {
        self.counterparties = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlVatUeGenerateDeclarationsResponseTotalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`section`](PlVatUeGenerateDeclarationsResponseTotalsItemBuilder::section)
    /// - [`counterparties`](PlVatUeGenerateDeclarationsResponseTotalsItemBuilder::counterparties)
    /// - [`amount`](PlVatUeGenerateDeclarationsResponseTotalsItemBuilder::amount)
    pub fn build(self) -> Result<PlVatUeGenerateDeclarationsResponseTotalsItem, BuildError> {
        Ok(PlVatUeGenerateDeclarationsResponseTotalsItem {
            section: self
                .section
                .ok_or_else(|| BuildError::missing_field("section"))?,
            counterparties: self
                .counterparties
                .ok_or_else(|| BuildError::missing_field("counterparties"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
