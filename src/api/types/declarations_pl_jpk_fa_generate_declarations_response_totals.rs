pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlJpkFaGenerateDeclarationsResponseTotals {
    #[serde(default)]
    pub invoices: String,
    #[serde(default)]
    pub lines: String,
}

impl PlJpkFaGenerateDeclarationsResponseTotals {
    pub fn builder() -> PlJpkFaGenerateDeclarationsResponseTotalsBuilder {
        <PlJpkFaGenerateDeclarationsResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlJpkFaGenerateDeclarationsResponseTotalsBuilder {
    invoices: Option<String>,
    lines: Option<String>,
}

impl PlJpkFaGenerateDeclarationsResponseTotalsBuilder {
    pub fn invoices(mut self, value: impl Into<String>) -> Self {
        self.invoices = Some(value.into());
        self
    }

    pub fn lines(mut self, value: impl Into<String>) -> Self {
        self.lines = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlJpkFaGenerateDeclarationsResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoices`](PlJpkFaGenerateDeclarationsResponseTotalsBuilder::invoices)
    /// - [`lines`](PlJpkFaGenerateDeclarationsResponseTotalsBuilder::lines)
    pub fn build(self) -> Result<PlJpkFaGenerateDeclarationsResponseTotals, BuildError> {
        Ok(PlJpkFaGenerateDeclarationsResponseTotals {
            invoices: self
                .invoices
                .ok_or_else(|| BuildError::missing_field("invoices"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
