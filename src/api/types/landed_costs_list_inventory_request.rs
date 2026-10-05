pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LandedCostsListInventoryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<LandedCostsListInventoryRequestSortItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<LandedCostsListInventoryRequestFilterItem>>,
    /// Numeric fields to sum over every row matching the filter (not only the current page)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals: Option<Vec<String>>,
}

impl LandedCostsListInventoryRequest {
    pub fn builder() -> LandedCostsListInventoryRequestBuilder {
        <LandedCostsListInventoryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandedCostsListInventoryRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    sort: Option<Vec<LandedCostsListInventoryRequestSortItem>>,
    filter: Option<Vec<LandedCostsListInventoryRequestFilterItem>>,
    totals: Option<Vec<String>>,
}

impl LandedCostsListInventoryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(mut self, value: Vec<LandedCostsListInventoryRequestSortItem>) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filter(mut self, value: Vec<LandedCostsListInventoryRequestFilterItem>) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn totals(mut self, value: Vec<String>) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandedCostsListInventoryRequest`].
    pub fn build(self) -> Result<LandedCostsListInventoryRequest, BuildError> {
        Ok(LandedCostsListInventoryRequest {
            page: self.page,
            page_size: self.page_size,
            sort: self.sort,
            filter: self.filter,
            totals: self.totals,
        })
    }
}
