pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlJpkFaGenerateDeclarationsResponseCounts {
    #[serde(default)]
    pub invoices: i64,
    #[serde(default)]
    pub lines: i64,
}

impl PlJpkFaGenerateDeclarationsResponseCounts {
    pub fn builder() -> PlJpkFaGenerateDeclarationsResponseCountsBuilder {
        <PlJpkFaGenerateDeclarationsResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlJpkFaGenerateDeclarationsResponseCountsBuilder {
    invoices: Option<i64>,
    lines: Option<i64>,
}

impl PlJpkFaGenerateDeclarationsResponseCountsBuilder {
    pub fn invoices(mut self, value: i64) -> Self {
        self.invoices = Some(value);
        self
    }

    pub fn lines(mut self, value: i64) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlJpkFaGenerateDeclarationsResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoices`](PlJpkFaGenerateDeclarationsResponseCountsBuilder::invoices)
    /// - [`lines`](PlJpkFaGenerateDeclarationsResponseCountsBuilder::lines)
    pub fn build(self) -> Result<PlJpkFaGenerateDeclarationsResponseCounts, BuildError> {
        Ok(PlJpkFaGenerateDeclarationsResponseCounts {
            invoices: self
                .invoices
                .ok_or_else(|| BuildError::missing_field("invoices"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
