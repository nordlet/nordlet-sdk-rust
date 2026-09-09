pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsFilesListRequest {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
}

impl PostV1CatalogItemsFilesListRequest {
    pub fn builder() -> PostV1CatalogItemsFilesListRequestBuilder {
        <PostV1CatalogItemsFilesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsFilesListRequestBuilder {
    item_id: Option<String>,
}

impl PostV1CatalogItemsFilesListRequestBuilder {
    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsFilesListRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_id`](PostV1CatalogItemsFilesListRequestBuilder::item_id)
    pub fn build(self) -> Result<PostV1CatalogItemsFilesListRequest, BuildError> {
        Ok(PostV1CatalogItemsFilesListRequest {
            item_id: self
                .item_id
                .ok_or_else(|| BuildError::missing_field("item_id"))?,
        })
    }
}
