pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    pub kind: PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItemKind,
    #[serde(default)]
    pub income: String,
}

impl PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItem {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItemBuilder {
        <PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItemBuilder {
    country_code: Option<String>,
    kind: Option<PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItemKind>,
    income: Option<String>,
}

impl PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn income(mut self, value: impl Into<String>) -> Self {
        self.income = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItemBuilder::country_code)
    /// - [`kind`](PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItemBuilder::kind)
    /// - [`income`](PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItemBuilder::income)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItem, BuildError> {
        Ok(
            PostV1DeclarationsDeReturnFactsSetResponseFactsForeignIncomeItem {
                country_code: self
                    .country_code
                    .ok_or_else(|| BuildError::missing_field("country_code"))?,
                kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
                income: self
                    .income
                    .ok_or_else(|| BuildError::missing_field("income"))?,
            },
        )
    }
}
