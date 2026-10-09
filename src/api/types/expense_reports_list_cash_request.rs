pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExpenseReportsListCashRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<ExpenseReportsListCashRequestSortItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<ExpenseReportsListCashRequestFilterItem>>,
    /// Numeric fields to sum over every row matching the filter (not only the current page)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals: Option<Vec<String>>,
}

impl ExpenseReportsListCashRequest {
    pub fn builder() -> ExpenseReportsListCashRequestBuilder {
        <ExpenseReportsListCashRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExpenseReportsListCashRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    sort: Option<Vec<ExpenseReportsListCashRequestSortItem>>,
    filter: Option<Vec<ExpenseReportsListCashRequestFilterItem>>,
    totals: Option<Vec<String>>,
}

impl ExpenseReportsListCashRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(mut self, value: Vec<ExpenseReportsListCashRequestSortItem>) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filter(mut self, value: Vec<ExpenseReportsListCashRequestFilterItem>) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn totals(mut self, value: Vec<String>) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExpenseReportsListCashRequest`].
    pub fn build(self) -> Result<ExpenseReportsListCashRequest, BuildError> {
        Ok(ExpenseReportsListCashRequest {
            page: self.page,
            page_size: self.page_size,
            sort: self.sort,
            filter: self.filter,
            totals: self.totals,
        })
    }
}
