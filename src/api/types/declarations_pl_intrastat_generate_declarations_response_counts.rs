pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlIntrastatGenerateDeclarationsResponseCounts {
    #[serde(default)]
    pub invoices: i64,
    #[serde(rename = "linesIncluded")]
    #[serde(default)]
    pub lines_included: i64,
    #[serde(rename = "linesSkipped")]
    #[serde(default)]
    pub lines_skipped: i64,
    #[serde(default)]
    pub returns: i64,
}

impl PlIntrastatGenerateDeclarationsResponseCounts {
    pub fn builder() -> PlIntrastatGenerateDeclarationsResponseCountsBuilder {
        <PlIntrastatGenerateDeclarationsResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlIntrastatGenerateDeclarationsResponseCountsBuilder {
    invoices: Option<i64>,
    lines_included: Option<i64>,
    lines_skipped: Option<i64>,
    returns: Option<i64>,
}

impl PlIntrastatGenerateDeclarationsResponseCountsBuilder {
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

    pub fn returns(mut self, value: i64) -> Self {
        self.returns = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PlIntrastatGenerateDeclarationsResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoices`](PlIntrastatGenerateDeclarationsResponseCountsBuilder::invoices)
    /// - [`lines_included`](PlIntrastatGenerateDeclarationsResponseCountsBuilder::lines_included)
    /// - [`lines_skipped`](PlIntrastatGenerateDeclarationsResponseCountsBuilder::lines_skipped)
    /// - [`returns`](PlIntrastatGenerateDeclarationsResponseCountsBuilder::returns)
    pub fn build(self) -> Result<PlIntrastatGenerateDeclarationsResponseCounts, BuildError> {
        Ok(PlIntrastatGenerateDeclarationsResponseCounts {
            invoices: self
                .invoices
                .ok_or_else(|| BuildError::missing_field("invoices"))?,
            lines_included: self
                .lines_included
                .ok_or_else(|| BuildError::missing_field("lines_included"))?,
            lines_skipped: self
                .lines_skipped
                .ok_or_else(|| BuildError::missing_field("lines_skipped"))?,
            returns: self
                .returns
                .ok_or_else(|| BuildError::missing_field("returns"))?,
        })
    }
}
