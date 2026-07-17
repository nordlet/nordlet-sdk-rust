pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsGetResponseTranslationsValue {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PostV1CatalogItemsGetResponseTranslationsValue {
    pub fn builder() -> PostV1CatalogItemsGetResponseTranslationsValueBuilder {
        <PostV1CatalogItemsGetResponseTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsGetResponseTranslationsValueBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl PostV1CatalogItemsGetResponseTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsGetResponseTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1CatalogItemsGetResponseTranslationsValueBuilder::name)
    pub fn build(self) -> Result<PostV1CatalogItemsGetResponseTranslationsValue, BuildError> {
        Ok(PostV1CatalogItemsGetResponseTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
        })
    }
}
