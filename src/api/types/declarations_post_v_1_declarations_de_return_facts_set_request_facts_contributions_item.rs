pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItem {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub date: String,
    pub kind: PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItemKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub amount: String,
}

impl PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItem {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItemBuilder {
        <PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItemBuilder {
    name: Option<String>,
    date: Option<String>,
    kind: Option<PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItemKind>,
    description: Option<String>,
    amount: Option<String>,
}

impl PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItemBuilder::name)
    /// - [`date`](PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItemBuilder::date)
    /// - [`kind`](PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItemBuilder::kind)
    /// - [`amount`](PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItemBuilder::amount)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItem, BuildError> {
        Ok(
            PostV1DeclarationsDeReturnFactsSetRequestFactsContributionsItem {
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
                date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
                kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
                description: self.description,
                amount: self
                    .amount
                    .ok_or_else(|| BuildError::missing_field("amount"))?,
            },
        )
    }
}
