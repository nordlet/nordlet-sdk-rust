pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1AgreementsAgreementsListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<PostV1AgreementsAgreementsListRequestSortItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<PostV1AgreementsAgreementsListRequestFilterItem>>,
}

impl PostV1AgreementsAgreementsListRequest {
    pub fn builder() -> PostV1AgreementsAgreementsListRequestBuilder {
        <PostV1AgreementsAgreementsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsListRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    sort: Option<Vec<PostV1AgreementsAgreementsListRequestSortItem>>,
    filter: Option<Vec<PostV1AgreementsAgreementsListRequestFilterItem>>,
}

impl PostV1AgreementsAgreementsListRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(mut self, value: Vec<PostV1AgreementsAgreementsListRequestSortItem>) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filter(mut self, value: Vec<PostV1AgreementsAgreementsListRequestFilterItem>) -> Self {
        self.filter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsListRequest`].
    pub fn build(self) -> Result<PostV1AgreementsAgreementsListRequest, BuildError> {
        Ok(PostV1AgreementsAgreementsListRequest {
            page: self.page,
            page_size: self.page_size,
            sort: self.sort,
            filter: self.filter,
        })
    }
}
