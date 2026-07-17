pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1EcommerceProductsListRequest {
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
    #[serde(rename = "priceListId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_list_id: Option<String>,
    #[serde(rename = "updatedSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_since: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl PostV1EcommerceProductsListRequest {
    pub fn builder() -> PostV1EcommerceProductsListRequestBuilder {
        <PostV1EcommerceProductsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1EcommerceProductsListRequestBuilder {
    warehouse_id: Option<String>,
    price_list_id: Option<String>,
    updated_since: Option<DateTime<FixedOffset>>,
    page: Option<i64>,
    page_size: Option<i64>,
}

impl PostV1EcommerceProductsListRequestBuilder {
    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    pub fn price_list_id(mut self, value: impl Into<String>) -> Self {
        self.price_list_id = Some(value.into());
        self
    }

    pub fn updated_since(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_since = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1EcommerceProductsListRequest`].
    pub fn build(self) -> Result<PostV1EcommerceProductsListRequest, BuildError> {
        Ok(PostV1EcommerceProductsListRequest {
            warehouse_id: self.warehouse_id,
            price_list_id: self.price_list_id,
            updated_since: self.updated_since,
            page: self.page,
            page_size: self.page_size,
        })
    }
}
