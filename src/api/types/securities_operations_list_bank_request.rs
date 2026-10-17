pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SecuritiesOperationsListBankRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<SecuritiesOperationsListBankRequestSortItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<SecuritiesOperationsListBankRequestFilterItem>>,
    /// Numeric fields to sum over every row matching the filter (not only the current page)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals: Option<Vec<String>>,
}

impl SecuritiesOperationsListBankRequest {
    pub fn builder() -> SecuritiesOperationsListBankRequestBuilder {
        <SecuritiesOperationsListBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SecuritiesOperationsListBankRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    sort: Option<Vec<SecuritiesOperationsListBankRequestSortItem>>,
    filter: Option<Vec<SecuritiesOperationsListBankRequestFilterItem>>,
    totals: Option<Vec<String>>,
}

impl SecuritiesOperationsListBankRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(mut self, value: Vec<SecuritiesOperationsListBankRequestSortItem>) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filter(mut self, value: Vec<SecuritiesOperationsListBankRequestFilterItem>) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn totals(mut self, value: Vec<String>) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SecuritiesOperationsListBankRequest`].
    pub fn build(self) -> Result<SecuritiesOperationsListBankRequest, BuildError> {
        Ok(SecuritiesOperationsListBankRequest {
            page: self.page,
            page_size: self.page_size,
            sort: self.sort,
            filter: self.filter,
            totals: self.totals,
        })
    }
}
