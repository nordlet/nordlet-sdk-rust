pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsGetResponseFactsContractsItem {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub date: String,
    #[serde(default)]
    pub partner: String,
    #[serde(default)]
    pub amount: String,
}

impl PostV1DeclarationsDeReturnFactsGetResponseFactsContractsItem {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsGetResponseFactsContractsItemBuilder {
        <PostV1DeclarationsDeReturnFactsGetResponseFactsContractsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsGetResponseFactsContractsItemBuilder {
    kind: Option<String>,
    date: Option<String>,
    partner: Option<String>,
    amount: Option<String>,
}

impl PostV1DeclarationsDeReturnFactsGetResponseFactsContractsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsGetResponseFactsContractsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](PostV1DeclarationsDeReturnFactsGetResponseFactsContractsItemBuilder::kind)
    /// - [`date`](PostV1DeclarationsDeReturnFactsGetResponseFactsContractsItemBuilder::date)
    /// - [`partner`](PostV1DeclarationsDeReturnFactsGetResponseFactsContractsItemBuilder::partner)
    /// - [`amount`](PostV1DeclarationsDeReturnFactsGetResponseFactsContractsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeReturnFactsGetResponseFactsContractsItem, BuildError> {
        Ok(
            PostV1DeclarationsDeReturnFactsGetResponseFactsContractsItem {
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
