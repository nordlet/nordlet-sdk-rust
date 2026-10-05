pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlJpkKrGenerateDeclarationsResponseTotals {
    #[serde(default)]
    pub operations: String,
    #[serde(default)]
    pub debit: String,
    #[serde(default)]
    pub credit: String,
}

impl PlJpkKrGenerateDeclarationsResponseTotals {
    pub fn builder() -> PlJpkKrGenerateDeclarationsResponseTotalsBuilder {
        <PlJpkKrGenerateDeclarationsResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlJpkKrGenerateDeclarationsResponseTotalsBuilder {
    operations: Option<String>,
    debit: Option<String>,
    credit: Option<String>,
}

impl PlJpkKrGenerateDeclarationsResponseTotalsBuilder {
    pub fn operations(mut self, value: impl Into<String>) -> Self {
        self.operations = Some(value.into());
        self
    }

    pub fn debit(mut self, value: impl Into<String>) -> Self {
        self.debit = Some(value.into());
        self
    }

    pub fn credit(mut self, value: impl Into<String>) -> Self {
        self.credit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlJpkKrGenerateDeclarationsResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`operations`](PlJpkKrGenerateDeclarationsResponseTotalsBuilder::operations)
    /// - [`debit`](PlJpkKrGenerateDeclarationsResponseTotalsBuilder::debit)
    /// - [`credit`](PlJpkKrGenerateDeclarationsResponseTotalsBuilder::credit)
    pub fn build(self) -> Result<PlJpkKrGenerateDeclarationsResponseTotals, BuildError> {
        Ok(PlJpkKrGenerateDeclarationsResponseTotals {
            operations: self
                .operations
                .ok_or_else(|| BuildError::missing_field("operations"))?,
            debit: self
                .debit
                .ok_or_else(|| BuildError::missing_field("debit"))?,
            credit: self
                .credit
                .ok_or_else(|| BuildError::missing_field("credit"))?,
        })
    }
}
