pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceiptsListPosRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<ReceiptsListPosRequestSortItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<ReceiptsListPosRequestFilterItem>>,
    /// Numeric fields to sum over every row matching the filter (not only the current page)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals: Option<Vec<String>>,
}

impl ReceiptsListPosRequest {
    pub fn builder() -> ReceiptsListPosRequestBuilder {
        <ReceiptsListPosRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsListPosRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    sort: Option<Vec<ReceiptsListPosRequestSortItem>>,
    filter: Option<Vec<ReceiptsListPosRequestFilterItem>>,
    totals: Option<Vec<String>>,
}

impl ReceiptsListPosRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(mut self, value: Vec<ReceiptsListPosRequestSortItem>) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filter(mut self, value: Vec<ReceiptsListPosRequestFilterItem>) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn totals(mut self, value: Vec<String>) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceiptsListPosRequest`].
    pub fn build(self) -> Result<ReceiptsListPosRequest, BuildError> {
        Ok(ReceiptsListPosRequest {
            page: self.page,
            page_size: self.page_size,
            sort: self.sort,
            filter: self.filter,
            totals: self.totals,
        })
    }
}
