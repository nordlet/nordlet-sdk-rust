pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtIntrastatComputeDeclarationsResponseCounts {
    #[serde(default)]
    pub invoices: i64,
    #[serde(rename = "linesIncluded")]
    #[serde(default)]
    pub lines_included: i64,
    #[serde(rename = "linesSkipped")]
    #[serde(default)]
    pub lines_skipped: i64,
}

impl LtIntrastatComputeDeclarationsResponseCounts {
    pub fn builder() -> LtIntrastatComputeDeclarationsResponseCountsBuilder {
        <LtIntrastatComputeDeclarationsResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIntrastatComputeDeclarationsResponseCountsBuilder {
    invoices: Option<i64>,
    lines_included: Option<i64>,
    lines_skipped: Option<i64>,
}

impl LtIntrastatComputeDeclarationsResponseCountsBuilder {
    pub fn invoices(mut self, value: i64) -> Self {
        self.invoices = Some(value);
        self
    }

    pub fn lines_included(mut self, value: i64) -> Self {
        self.lines_included = Some(value);
        self
    }

    pub fn lines_skipped(mut self, value: i64) -> Self {
        self.lines_skipped = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtIntrastatComputeDeclarationsResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoices`](LtIntrastatComputeDeclarationsResponseCountsBuilder::invoices)
    /// - [`lines_included`](LtIntrastatComputeDeclarationsResponseCountsBuilder::lines_included)
    /// - [`lines_skipped`](LtIntrastatComputeDeclarationsResponseCountsBuilder::lines_skipped)
    pub fn build(self) -> Result<LtIntrastatComputeDeclarationsResponseCounts, BuildError> {
        Ok(LtIntrastatComputeDeclarationsResponseCounts {
            invoices: self
                .invoices
                .ok_or_else(|| BuildError::missing_field("invoices"))?,
            lines_included: self
                .lines_included
                .ok_or_else(|| BuildError::missing_field("lines_included"))?,
            lines_skipped: self
                .lines_skipped
                .ok_or_else(|| BuildError::missing_field("lines_skipped"))?,
        })
    }
}
