pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TypesListAgreementsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<TypesListAgreementsRequestSortItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<TypesListAgreementsRequestFilterItem>>,
    /// Numeric fields to sum over every row matching the filter (not only the current page)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals: Option<Vec<String>>,
}

impl TypesListAgreementsRequest {
    pub fn builder() -> TypesListAgreementsRequestBuilder {
        <TypesListAgreementsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesListAgreementsRequestBuilder {
    page: Option<i64>,
    page_size: Option<i64>,
    sort: Option<Vec<TypesListAgreementsRequestSortItem>>,
    filter: Option<Vec<TypesListAgreementsRequestFilterItem>>,
    totals: Option<Vec<String>>,
}

impl TypesListAgreementsRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn sort(mut self, value: Vec<TypesListAgreementsRequestSortItem>) -> Self {
        self.sort = Some(value);
        self
    }

    pub fn filter(mut self, value: Vec<TypesListAgreementsRequestFilterItem>) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn totals(mut self, value: Vec<String>) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TypesListAgreementsRequest`].
    pub fn build(self) -> Result<TypesListAgreementsRequest, BuildError> {
        Ok(TypesListAgreementsRequest {
            page: self.page,
            page_size: self.page_size,
            sort: self.sort,
            filter: self.filter,
            totals: self.totals,
        })
    }
}
