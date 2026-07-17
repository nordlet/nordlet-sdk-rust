pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsGeneralJournalRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
}

impl PostV1ReportsGeneralJournalRequest {
    pub fn builder() -> PostV1ReportsGeneralJournalRequestBuilder {
        <PostV1ReportsGeneralJournalRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsGeneralJournalRequestBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
}

impl PostV1ReportsGeneralJournalRequestBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1ReportsGeneralJournalRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsGeneralJournalRequestBuilder::from_date)
    /// - [`to_date`](PostV1ReportsGeneralJournalRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1ReportsGeneralJournalRequest, BuildError> {
        Ok(PostV1ReportsGeneralJournalRequest {
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
