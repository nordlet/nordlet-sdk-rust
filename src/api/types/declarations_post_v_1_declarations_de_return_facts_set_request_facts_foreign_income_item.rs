pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    pub kind: PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItemKind,
    #[serde(default)]
    pub income: String,
}

impl PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItem {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItemBuilder {
        <PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItemBuilder {
    country_code: Option<String>,
    kind: Option<PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItemKind>,
    income: Option<String>,
}

impl PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn income(mut self, value: impl Into<String>) -> Self {
        self.income = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItemBuilder::country_code)
    /// - [`kind`](PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItemBuilder::kind)
    /// - [`income`](PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItemBuilder::income)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItem, BuildError> {
        Ok(
            PostV1DeclarationsDeReturnFactsSetRequestFactsForeignIncomeItem {
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
