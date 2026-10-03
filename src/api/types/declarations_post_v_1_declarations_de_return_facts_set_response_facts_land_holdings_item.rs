pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItem {
    #[serde(rename = "fileNumber")]
    #[serde(default)]
    pub file_number: String,
    #[serde(rename = "assessedValue")]
    #[serde(default)]
    pub assessed_value: String,
    pub category: PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItemCategory,
}

impl PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItem {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItemBuilder {
        <PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItemBuilder {
    file_number: Option<String>,
    assessed_value: Option<String>,
    category: Option<PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItemCategory>,
}

impl PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItemBuilder {
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
        value: PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItemCategory,
    ) -> Self {
        self.category = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_number`](PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItemBuilder::file_number)
    /// - [`assessed_value`](PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItemBuilder::assessed_value)
    /// - [`category`](PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItemBuilder::category)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItem, BuildError> {
        Ok(
            PostV1DeclarationsDeReturnFactsSetResponseFactsLandHoldingsItem {
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
