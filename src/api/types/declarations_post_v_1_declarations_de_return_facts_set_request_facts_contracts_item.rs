pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsSetRequestFactsContractsItem {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub partner: String,
    #[serde(default)]
    pub amount: String,
}

impl PostV1DeclarationsDeReturnFactsSetRequestFactsContractsItem {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsSetRequestFactsContractsItemBuilder {
        <PostV1DeclarationsDeReturnFactsSetRequestFactsContractsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsSetRequestFactsContractsItemBuilder {
    kind: Option<String>,
    date: Option<String>,
    partner: Option<String>,
    amount: Option<String>,
}

impl PostV1DeclarationsDeReturnFactsSetRequestFactsContractsItemBuilder {
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn partner(mut self, value: impl Into<String>) -> Self {
        self.partner = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsSetRequestFactsContractsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](PostV1DeclarationsDeReturnFactsSetRequestFactsContractsItemBuilder::kind)
    /// - [`date`](PostV1DeclarationsDeReturnFactsSetRequestFactsContractsItemBuilder::date)
    /// - [`partner`](PostV1DeclarationsDeReturnFactsSetRequestFactsContractsItemBuilder::partner)
    /// - [`amount`](PostV1DeclarationsDeReturnFactsSetRequestFactsContractsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeReturnFactsSetRequestFactsContractsItem, BuildError> {
        Ok(
            PostV1DeclarationsDeReturnFactsSetRequestFactsContractsItem {
                kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
                date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
                partner: self
                    .partner
                    .ok_or_else(|| BuildError::missing_field("partner"))?,
                amount: self
                    .amount
                    .ok_or_else(|| BuildError::missing_field("amount"))?,
            },
        )
    }
}
