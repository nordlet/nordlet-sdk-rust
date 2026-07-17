pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1WebhooksDeliveriesListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<PostV1WebhooksDeliveriesListRequestSortItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<PostV1WebhooksDeliveriesListRequestFilterItem>>,
}

impl PostV1WebhooksDeliveriesListRequest {
    pub fn builder() -> PostV1WebhooksDeliveriesListRequestBuilder {
        <PostV1WebhooksDeliveriesListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1WebhooksDeliveriesListRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    sort: Option<Vec<PostV1WebhooksDeliveriesListRequestSortItem>>,
    filter: Option<Vec<PostV1WebhooksDeliveriesListRequestFilterItem>>,
}

impl PostV1WebhooksDeliveriesListRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(mut self, value: Vec<PostV1WebhooksDeliveriesListRequestSortItem>) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filter(mut self, value: Vec<PostV1WebhooksDeliveriesListRequestFilterItem>) -> Self {
        self.filter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1WebhooksDeliveriesListRequest`].
    pub fn build(self) -> Result<PostV1WebhooksDeliveriesListRequest, BuildError> {
        Ok(PostV1WebhooksDeliveriesListRequest {
            page: self.page,
            page_size: self.page_size,
            sort: self.sort,
            filter: self.filter,
        })
    }
}
