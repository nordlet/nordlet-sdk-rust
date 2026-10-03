pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlJpkKrGenerateResponseTotals {
    #[serde(default)]
    pub operations: String,
    #[serde(default)]
    pub debit: String,
    #[serde(default)]
    pub credit: String,
}

impl PostV1DeclarationsPlJpkKrGenerateResponseTotals {
    pub fn builder() -> PostV1DeclarationsPlJpkKrGenerateResponseTotalsBuilder {
        <PostV1DeclarationsPlJpkKrGenerateResponseTotalsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlJpkKrGenerateResponseTotalsBuilder {
    operations: Option<String>,
    debit: Option<String>,
    credit: Option<String>,
}

impl PostV1DeclarationsPlJpkKrGenerateResponseTotalsBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlJpkKrGenerateResponseTotals`].
    /// This method will fail if any of the following fields are not set:
    /// - [`operations`](PostV1DeclarationsPlJpkKrGenerateResponseTotalsBuilder::operations)
    /// - [`debit`](PostV1DeclarationsPlJpkKrGenerateResponseTotalsBuilder::debit)
    /// - [`credit`](PostV1DeclarationsPlJpkKrGenerateResponseTotalsBuilder::credit)
    pub fn build(self) -> Result<PostV1DeclarationsPlJpkKrGenerateResponseTotals, BuildError> {
        Ok(PostV1DeclarationsPlJpkKrGenerateResponseTotals {
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
