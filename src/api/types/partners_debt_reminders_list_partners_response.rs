pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DebtRemindersListPartnersResponse {
    #[serde(default)]
    pub rows: Vec<DebtRemindersListPartnersResponseRowsItem>,
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

impl DebtRemindersListPartnersResponse {
    pub fn builder() -> DebtRemindersListPartnersResponseBuilder {
        <DebtRemindersListPartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtRemindersListPartnersResponseBuilder {
    rows: Option<Vec<DebtRemindersListPartnersResponseRowsItem>>,
    page: Option<i64>,
    page_size: Option<i64>,
    total: Option<i64>,
    totals: Option<HashMap<String, String>>,
}

impl DebtRemindersListPartnersResponseBuilder {
    pub fn rows(mut self, value: Vec<DebtRemindersListPartnersResponseRowsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`DebtRemindersListPartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](DebtRemindersListPartnersResponseBuilder::rows)
    /// - [`page`](DebtRemindersListPartnersResponseBuilder::page)
    /// - [`page_size`](DebtRemindersListPartnersResponseBuilder::page_size)
    /// - [`total`](DebtRemindersListPartnersResponseBuilder::total)
    pub fn build(self) -> Result<DebtRemindersListPartnersResponse, BuildError> {
        Ok(DebtRemindersListPartnersResponse {
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
