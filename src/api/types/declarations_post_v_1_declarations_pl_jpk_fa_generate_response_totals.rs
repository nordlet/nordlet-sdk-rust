pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlJpkFaGenerateResponseTotals {
    #[serde(default)]
    pub invoices: String,
    #[serde(default)]
    pub lines: String,
}

impl PostV1DeclarationsPlJpkFaGenerateResponseTotals {
    pub fn builder() -> PostV1DeclarationsPlJpkFaGenerateResponseTotalsBuilder {
        <PostV1DeclarationsPlJpkFaGenerateResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlJpkFaGenerateResponseTotalsBuilder {
    invoices: Option<String>,
    lines: Option<String>,
}

impl PostV1DeclarationsPlJpkFaGenerateResponseTotalsBuilder {
    pub fn invoices(mut self, value: impl Into<String>) -> Self {
        self.invoices = Some(value.into());
        self
    }

    pub fn lines(mut self, value: impl Into<String>) -> Self {
        self.lines = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlJpkFaGenerateResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoices`](PostV1DeclarationsPlJpkFaGenerateResponseTotalsBuilder::invoices)
    /// - [`lines`](PostV1DeclarationsPlJpkFaGenerateResponseTotalsBuilder::lines)
    pub fn build(self) -> Result<PostV1DeclarationsPlJpkFaGenerateResponseTotals, BuildError> {
        Ok(PostV1DeclarationsPlJpkFaGenerateResponseTotals {
            invoices: self
                .invoices
                .ok_or_else(|| BuildError::missing_field("invoices"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
