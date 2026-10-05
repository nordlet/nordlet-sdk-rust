pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProductsListEcommerceResponse {
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub rows: Vec<ProductsListEcommerceResponseRowsItem>,
}

impl ProductsListEcommerceResponse {
    pub fn builder() -> ProductsListEcommerceResponseBuilder {
        <ProductsListEcommerceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductsListEcommerceResponseBuilder {
    total: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
    rows: Option<Vec<ProductsListEcommerceResponseRowsItem>>,
}

impl ProductsListEcommerceResponseBuilder {
    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
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

    pub fn rows(mut self, value: Vec<ProductsListEcommerceResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProductsListEcommerceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total`](ProductsListEcommerceResponseBuilder::total)
    /// - [`page`](ProductsListEcommerceResponseBuilder::page)
    /// - [`page_size`](ProductsListEcommerceResponseBuilder::page_size)
    /// - [`rows`](ProductsListEcommerceResponseBuilder::rows)
    pub fn build(self) -> Result<ProductsListEcommerceResponse, BuildError> {
        Ok(ProductsListEcommerceResponse {
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
