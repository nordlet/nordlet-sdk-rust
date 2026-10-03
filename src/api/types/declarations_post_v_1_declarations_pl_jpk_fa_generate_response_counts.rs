pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlJpkFaGenerateResponseCounts {
    #[serde(default)]
    pub invoices: i64,
    #[serde(default)]
    pub lines: i64,
}

impl PostV1DeclarationsPlJpkFaGenerateResponseCounts {
    pub fn builder() -> PostV1DeclarationsPlJpkFaGenerateResponseCountsBuilder {
        <PostV1DeclarationsPlJpkFaGenerateResponseCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlJpkFaGenerateResponseCountsBuilder {
    invoices: Option<i64>,
    lines: Option<i64>,
}

impl PostV1DeclarationsPlJpkFaGenerateResponseCountsBuilder {
    pub fn invoices(mut self, value: i64) -> Self {
        self.invoices = Some(value);
        self
    }

    pub fn lines(mut self, value: i64) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlJpkFaGenerateResponseCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoices`](PostV1DeclarationsPlJpkFaGenerateResponseCountsBuilder::invoices)
    /// - [`lines`](PostV1DeclarationsPlJpkFaGenerateResponseCountsBuilder::lines)
    pub fn build(self) -> Result<PostV1DeclarationsPlJpkFaGenerateResponseCounts, BuildError> {
        Ok(PostV1DeclarationsPlJpkFaGenerateResponseCounts {
            invoices: self
                .invoices
                .ok_or_else(|| BuildError::missing_field("invoices"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
