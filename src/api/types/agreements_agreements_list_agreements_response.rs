pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgreementsListAgreementsResponse {
    #[serde(default)]
    pub rows: Vec<AgreementsListAgreementsResponseRowsItem>,
    #[serde(default)]
    pub page: i64,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: i64,
    #[serde(default)]
    pub total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals: Option<HashMap<String, String>>,
}

impl AgreementsListAgreementsResponse {
    pub fn builder() -> AgreementsListAgreementsResponseBuilder {
        <AgreementsListAgreementsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgreementsListAgreementsResponseBuilder {
    rows: Option<Vec<AgreementsListAgreementsResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
    totals: Option<HashMap<String, String>>,
}

impl AgreementsListAgreementsResponseBuilder {
    pub fn rows(mut self, value: Vec<AgreementsListAgreementsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn totals(mut self, value: HashMap<String, String>) -> Self {
        self.totals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgreementsListAgreementsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](AgreementsListAgreementsResponseBuilder::rows)
    /// - [`page`](AgreementsListAgreementsResponseBuilder::page)
    /// - [`page_size`](AgreementsListAgreementsResponseBuilder::page_size)
    /// - [`total`](AgreementsListAgreementsResponseBuilder::total)
    pub fn build(self) -> Result<AgreementsListAgreementsResponse, BuildError> {
        Ok(AgreementsListAgreementsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            page: self.page.ok_or_else(|| BuildError::missing_field("page"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            totals: self.totals,
        })
    }
}
