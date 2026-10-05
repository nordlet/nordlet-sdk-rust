pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeneralJournalReportsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl GeneralJournalReportsRequest {
    pub fn builder() -> GeneralJournalReportsRequestBuilder {
        <GeneralJournalReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneralJournalReportsRequestBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    page: Option<i64>,
    page_size: Option<i64>,
}

impl GeneralJournalReportsRequestBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
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

    /// Consumes the builder and constructs a [`GeneralJournalReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](GeneralJournalReportsRequestBuilder::from_date)
    /// - [`to_date`](GeneralJournalReportsRequestBuilder::to_date)
    pub fn build(self) -> Result<GeneralJournalReportsRequest, BuildError> {
        Ok(GeneralJournalReportsRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            page: self.page,
            page_size: self.page_size,
        })
    }
}
