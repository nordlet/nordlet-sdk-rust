pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1CatalogItemsKindsUpdateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "saftType")]
    pub saft_type: PostV1CatalogItemsKindsUpdateResponseSaftType,
    #[serde(rename = "quantityAccounting")]
    #[serde(default)]
    pub quantity_accounting: bool,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    pub sort_order: i64,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl PostV1CatalogItemsKindsUpdateResponse {
    pub fn builder() -> PostV1CatalogItemsKindsUpdateResponseBuilder {
        <PostV1CatalogItemsKindsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogItemsKindsUpdateResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    saft_type: Option<PostV1CatalogItemsKindsUpdateResponseSaftType>,
    quantity_accounting: Option<bool>,
    sort_order: Option<i64>,
    created_at: Option<String>,
}

impl PostV1CatalogItemsKindsUpdateResponseBuilder {
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

    pub fn saft_type(mut self, value: PostV1CatalogItemsKindsUpdateResponseSaftType) -> Self {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogItemsKindsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogItemsKindsUpdateResponseBuilder::id)
    /// - [`code`](PostV1CatalogItemsKindsUpdateResponseBuilder::code)
    /// - [`name`](PostV1CatalogItemsKindsUpdateResponseBuilder::name)
    /// - [`saft_type`](PostV1CatalogItemsKindsUpdateResponseBuilder::saft_type)
    /// - [`quantity_accounting`](PostV1CatalogItemsKindsUpdateResponseBuilder::quantity_accounting)
    /// - [`sort_order`](PostV1CatalogItemsKindsUpdateResponseBuilder::sort_order)
    /// - [`created_at`](PostV1CatalogItemsKindsUpdateResponseBuilder::created_at)
    pub fn build(self) -> Result<PostV1CatalogItemsKindsUpdateResponse, BuildError> {
        Ok(PostV1CatalogItemsKindsUpdateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            saft_type: self
                .saft_type
                .ok_or_else(|| BuildError::missing_field("saft_type"))?,
            quantity_accounting: self
                .quantity_accounting
                .ok_or_else(|| BuildError::missing_field("quantity_accounting"))?,
            sort_order: self
                .sort_order
                .ok_or_else(|| BuildError::missing_field("sort_order"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
