pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlVatUeGenerateResponseTotalsItem {
    pub section: PostV1DeclarationsPlVatUeGenerateResponseTotalsItemSection,
    #[serde(default)]
    pub counterparties: i64,
    #[serde(default)]
    pub amount: String,
}

impl PostV1DeclarationsPlVatUeGenerateResponseTotalsItem {
    pub fn builder() -> PostV1DeclarationsPlVatUeGenerateResponseTotalsItemBuilder {
        <PostV1DeclarationsPlVatUeGenerateResponseTotalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlVatUeGenerateResponseTotalsItemBuilder {
    section: Option<PostV1DeclarationsPlVatUeGenerateResponseTotalsItemSection>,
    counterparties: Option<i64>,
    amount: Option<String>,
}

impl PostV1DeclarationsPlVatUeGenerateResponseTotalsItemBuilder {
    pub fn section(
        mut self,
        value: PostV1DeclarationsPlVatUeGenerateResponseTotalsItemSection,
    ) -> Self {
        self.section = Some(value);
        self
    }

    pub fn counterparties(mut self, value: i64) -> Self {
        self.counterparties = Some(value);
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlVatUeGenerateResponseTotalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`section`](PostV1DeclarationsPlVatUeGenerateResponseTotalsItemBuilder::section)
    /// - [`counterparties`](PostV1DeclarationsPlVatUeGenerateResponseTotalsItemBuilder::counterparties)
    /// - [`amount`](PostV1DeclarationsPlVatUeGenerateResponseTotalsItemBuilder::amount)
    pub fn build(self) -> Result<PostV1DeclarationsPlVatUeGenerateResponseTotalsItem, BuildError> {
        Ok(PostV1DeclarationsPlVatUeGenerateResponseTotalsItem {
            section: self
                .section
                .ok_or_else(|| BuildError::missing_field("section"))?,
            counterparties: self
                .counterparties
                .ok_or_else(|| BuildError::missing_field("counterparties"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
        })
    }
}
