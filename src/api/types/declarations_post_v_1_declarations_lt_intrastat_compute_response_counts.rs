pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIntrastatComputeResponseCounts {
    #[serde(default)]
    pub invoices: i64,
    #[serde(rename = "linesIncluded")]
    #[serde(default)]
    pub lines_included: i64,
    #[serde(rename = "linesSkipped")]
    #[serde(default)]
    pub lines_skipped: i64,
}

impl PostV1DeclarationsLtIntrastatComputeResponseCounts {
    pub fn builder() -> PostV1DeclarationsLtIntrastatComputeResponseCountsBuilder {
        <PostV1DeclarationsLtIntrastatComputeResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIntrastatComputeResponseCountsBuilder {
    invoices: Option<i64>,
    lines_included: Option<i64>,
    lines_skipped: Option<i64>,
}

impl PostV1DeclarationsLtIntrastatComputeResponseCountsBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIntrastatComputeResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoices`](PostV1DeclarationsLtIntrastatComputeResponseCountsBuilder::invoices)
    /// - [`lines_included`](PostV1DeclarationsLtIntrastatComputeResponseCountsBuilder::lines_included)
    /// - [`lines_skipped`](PostV1DeclarationsLtIntrastatComputeResponseCountsBuilder::lines_skipped)
    pub fn build(self) -> Result<PostV1DeclarationsLtIntrastatComputeResponseCounts, BuildError> {
        Ok(PostV1DeclarationsLtIntrastatComputeResponseCounts {
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
