pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItem {
    #[serde(rename = "fileNumber")]
    #[serde(default)]
    pub file_number: String,
    #[serde(rename = "assessedValue")]
    #[serde(default)]
    pub assessed_value: String,
    pub category: PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItemCategory,
}

impl PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItem {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItemBuilder {
        <PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItemBuilder {
    file_number: Option<String>,
    assessed_value: Option<String>,
    category: Option<PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItemCategory>,
}

impl PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItemBuilder {
    pub fn file_number(mut self, value: impl Into<String>) -> Self {
        self.file_number = Some(value.into());
        self
    }

    pub fn assessed_value(mut self, value: impl Into<String>) -> Self {
        self.assessed_value = Some(value.into());
        self
    }

    pub fn category(
        mut self,
        value: PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItemCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_number`](PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItemBuilder::file_number)
    /// - [`assessed_value`](PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItemBuilder::assessed_value)
    /// - [`category`](PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItemBuilder::category)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItem, BuildError> {
        Ok(
            PostV1DeclarationsDeReturnFactsSetRequestFactsLandHoldingsItem {
                file_number: self
                    .file_number
                    .ok_or_else(|| BuildError::missing_field("file_number"))?,
                assessed_value: self
                    .assessed_value
                    .ok_or_else(|| BuildError::missing_field("assessed_value"))?,
                category: self
                    .category
                    .ok_or_else(|| BuildError::missing_field("category"))?,
            },
        )
    }
}
