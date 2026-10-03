pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlIntrastatGenerateResponseCounts {
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

impl PostV1DeclarationsPlIntrastatGenerateResponseCounts {
    pub fn builder() -> PostV1DeclarationsPlIntrastatGenerateResponseCountsBuilder {
        <PostV1DeclarationsPlIntrastatGenerateResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlIntrastatGenerateResponseCountsBuilder {
    invoices: Option<i64>,
    lines_included: Option<i64>,
    lines_skipped: Option<i64>,
    returns: Option<i64>,
}

impl PostV1DeclarationsPlIntrastatGenerateResponseCountsBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlIntrastatGenerateResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoices`](PostV1DeclarationsPlIntrastatGenerateResponseCountsBuilder::invoices)
    /// - [`lines_included`](PostV1DeclarationsPlIntrastatGenerateResponseCountsBuilder::lines_included)
    /// - [`lines_skipped`](PostV1DeclarationsPlIntrastatGenerateResponseCountsBuilder::lines_skipped)
    /// - [`returns`](PostV1DeclarationsPlIntrastatGenerateResponseCountsBuilder::returns)
    pub fn build(self) -> Result<PostV1DeclarationsPlIntrastatGenerateResponseCounts, BuildError> {
        Ok(PostV1DeclarationsPlIntrastatGenerateResponseCounts {
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
