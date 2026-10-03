pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    pub kind: PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItemKind,
    #[serde(default)]
    pub income: String,
}

impl PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItem {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItemBuilder {
        <PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItemBuilder {
    country_code: Option<String>,
    kind: Option<PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItemKind>,
    income: Option<String>,
}

impl PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn kind(
        mut self,
        value: PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItemKind,
    ) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn income(mut self, value: impl Into<String>) -> Self {
        self.income = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItemBuilder::country_code)
    /// - [`kind`](PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItemBuilder::kind)
    /// - [`income`](PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItemBuilder::income)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItem, BuildError> {
        Ok(
            PostV1DeclarationsDeReturnFactsGetResponseFactsForeignIncomeItem {
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
