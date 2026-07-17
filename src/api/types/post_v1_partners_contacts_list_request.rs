pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1PartnersContactsListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<PostV1PartnersContactsListRequestSortItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<PostV1PartnersContactsListRequestFilterItem>>,
}

impl PostV1PartnersContactsListRequest {
    pub fn builder() -> PostV1PartnersContactsListRequestBuilder {
        <PostV1PartnersContactsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersContactsListRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    sort: Option<Vec<PostV1PartnersContactsListRequestSortItem>>,
    filter: Option<Vec<PostV1PartnersContactsListRequestFilterItem>>,
}

impl PostV1PartnersContactsListRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(mut self, value: Vec<PostV1PartnersContactsListRequestSortItem>) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filter(mut self, value: Vec<PostV1PartnersContactsListRequestFilterItem>) -> Self {
        self.filter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersContactsListRequest`].
    pub fn build(self) -> Result<PostV1PartnersContactsListRequest, BuildError> {
        Ok(PostV1PartnersContactsListRequest {
            page: self.page,
            page_size: self.page_size,
            sort: self.sort,
            filter: self.filter,
        })
    }
}
