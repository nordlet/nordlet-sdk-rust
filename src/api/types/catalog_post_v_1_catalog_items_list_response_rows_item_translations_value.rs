pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsListResponseRowsItemTranslationsValue {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PostV1CatalogItemsListResponseRowsItemTranslationsValue {
    pub fn builder() -> PostV1CatalogItemsListResponseRowsItemTranslationsValueBuilder {
        <PostV1CatalogItemsListResponseRowsItemTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsListResponseRowsItemTranslationsValueBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl PostV1CatalogItemsListResponseRowsItemTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsListResponseRowsItemTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1CatalogItemsListResponseRowsItemTranslationsValueBuilder::name)
    pub fn build(
        self,
    ) -> Result<PostV1CatalogItemsListResponseRowsItemTranslationsValue, BuildError> {
        Ok(PostV1CatalogItemsListResponseRowsItemTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
        })
    }
}
