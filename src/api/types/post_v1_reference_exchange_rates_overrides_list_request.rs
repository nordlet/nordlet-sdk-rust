pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1ReferenceExchangeRatesOverridesListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<PostV1ReferenceExchangeRatesOverridesListRequestSortItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<PostV1ReferenceExchangeRatesOverridesListRequestFilterItem>>,
}

impl PostV1ReferenceExchangeRatesOverridesListRequest {
    pub fn builder() -> PostV1ReferenceExchangeRatesOverridesListRequestBuilder {
        <PostV1ReferenceExchangeRatesOverridesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceExchangeRatesOverridesListRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    sort: Option<Vec<PostV1ReferenceExchangeRatesOverridesListRequestSortItem>>,
    filter: Option<Vec<PostV1ReferenceExchangeRatesOverridesListRequestFilterItem>>,
}

impl PostV1ReferenceExchangeRatesOverridesListRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(
        mut self,
        value: Vec<PostV1ReferenceExchangeRatesOverridesListRequestSortItem>,
    ) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filter(
        mut self,
        value: Vec<PostV1ReferenceExchangeRatesOverridesListRequestFilterItem>,
    ) -> Self {
        self.filter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceExchangeRatesOverridesListRequest`].
    pub fn build(self) -> Result<PostV1ReferenceExchangeRatesOverridesListRequest, BuildError> {
        Ok(PostV1ReferenceExchangeRatesOverridesListRequest {
            page: self.page,
            page_size: self.page_size,
            sort: self.sort,
            filter: self.filter,
        })
    }
}
