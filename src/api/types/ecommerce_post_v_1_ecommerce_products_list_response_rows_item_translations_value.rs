pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceProductsListResponseRowsItemTranslationsValue {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PostV1EcommerceProductsListResponseRowsItemTranslationsValue {
    pub fn builder() -> PostV1EcommerceProductsListResponseRowsItemTranslationsValueBuilder {
        <PostV1EcommerceProductsListResponseRowsItemTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceProductsListResponseRowsItemTranslationsValueBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl PostV1EcommerceProductsListResponseRowsItemTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceProductsListResponseRowsItemTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1EcommerceProductsListResponseRowsItemTranslationsValueBuilder::name)
    pub fn build(
        self,
    ) -> Result<PostV1EcommerceProductsListResponseRowsItemTranslationsValue, BuildError> {
        Ok(
            PostV1EcommerceProductsListResponseRowsItemTranslationsValue {
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
                description: self.description,
            },
        )
    }
}
