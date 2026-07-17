pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1WebhooksSubscriptionsListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<PostV1WebhooksSubscriptionsListRequestSortItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<PostV1WebhooksSubscriptionsListRequestFilterItem>>,
}

impl PostV1WebhooksSubscriptionsListRequest {
    pub fn builder() -> PostV1WebhooksSubscriptionsListRequestBuilder {
        <PostV1WebhooksSubscriptionsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1WebhooksSubscriptionsListRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    sort: Option<Vec<PostV1WebhooksSubscriptionsListRequestSortItem>>,
    filter: Option<Vec<PostV1WebhooksSubscriptionsListRequestFilterItem>>,
}

impl PostV1WebhooksSubscriptionsListRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(mut self, value: Vec<PostV1WebhooksSubscriptionsListRequestSortItem>) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filter(mut self, value: Vec<PostV1WebhooksSubscriptionsListRequestFilterItem>) -> Self {
        self.filter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1WebhooksSubscriptionsListRequest`].
    pub fn build(self) -> Result<PostV1WebhooksSubscriptionsListRequest, BuildError> {
        Ok(PostV1WebhooksSubscriptionsListRequest {
            page: self.page,
            page_size: self.page_size,
            sort: self.sort,
            filter: self.filter,
        })
    }
}
