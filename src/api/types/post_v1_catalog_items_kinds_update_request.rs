pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsKindsUpdateRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "saftType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saft_type: Option<PostV1CatalogItemsKindsUpdateRequestSaftType>,
    #[serde(rename = "quantityAccounting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_accounting: Option<bool>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i64>,
}

impl PostV1CatalogItemsKindsUpdateRequest {
    pub fn builder() -> PostV1CatalogItemsKindsUpdateRequestBuilder {
        <PostV1CatalogItemsKindsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsKindsUpdateRequestBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    saft_type: Option<PostV1CatalogItemsKindsUpdateRequestSaftType>,
    quantity_accounting: Option<bool>,
    sort_order: Option<i64>,
}

impl PostV1CatalogItemsKindsUpdateRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn saft_type(mut self, value: PostV1CatalogItemsKindsUpdateRequestSaftType) -> Self {
        self.saft_type = Some(value);
        self
    }

    pub fn quantity_accounting(mut self, value: bool) -> Self {
        self.quantity_accounting = Some(value);
        self
    }

    pub fn sort_order(mut self, value: i64) -> Self {
        self.sort_order = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsKindsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemsKindsUpdateRequestBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogItemsKindsUpdateRequest, BuildError> {
        Ok(PostV1CatalogItemsKindsUpdateRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code,
            name: self.name,
            saft_type: self.saft_type,
            quantity_accounting: self.quantity_accounting,
            sort_order: self.sort_order,
        })
    }
}
