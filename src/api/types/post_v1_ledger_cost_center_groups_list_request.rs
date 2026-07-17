pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1LedgerCostCenterGroupsListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<PostV1LedgerCostCenterGroupsListRequestSortItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<PostV1LedgerCostCenterGroupsListRequestFilterItem>>,
}

impl PostV1LedgerCostCenterGroupsListRequest {
    pub fn builder() -> PostV1LedgerCostCenterGroupsListRequestBuilder {
        <PostV1LedgerCostCenterGroupsListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerCostCenterGroupsListRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    sort: Option<Vec<PostV1LedgerCostCenterGroupsListRequestSortItem>>,
    filter: Option<Vec<PostV1LedgerCostCenterGroupsListRequestFilterItem>>,
}

impl PostV1LedgerCostCenterGroupsListRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(mut self, value: Vec<PostV1LedgerCostCenterGroupsListRequestSortItem>) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filter(mut self, value: Vec<PostV1LedgerCostCenterGroupsListRequestFilterItem>) -> Self {
        self.filter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerCostCenterGroupsListRequest`].
    pub fn build(self) -> Result<PostV1LedgerCostCenterGroupsListRequest, BuildError> {
        Ok(PostV1LedgerCostCenterGroupsListRequest {
            page: self.page,
            page_size: self.page_size,
            sort: self.sort,
            filter: self.filter,
        })
    }
}
