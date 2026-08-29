pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1BillingAccountGetResponseMonthToDate {
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
    #[serde(rename = "apiRequests")]
    #[serde(default)]
    pub api_requests: i64,
    #[serde(rename = "ocrPages")]
    #[serde(default)]
    pub ocr_pages: i64,
    #[serde(rename = "fileBytes")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub file_bytes: f64,
    #[serde(rename = "databaseBytes")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub database_bytes: f64,
    #[serde(rename = "archivedCompanies")]
    #[serde(default)]
    pub archived_companies: i64,
    #[serde(rename = "estimatedTodayCents")]
    #[serde(default)]
    pub estimated_today_cents: i64,
}

impl PostV1BillingAccountGetResponseMonthToDate {
    pub fn builder() -> PostV1BillingAccountGetResponseMonthToDateBuilder {
        <PostV1BillingAccountGetResponseMonthToDateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingAccountGetResponseMonthToDateBuilder {
    from: Option<String>,
    to: Option<String>,
    api_requests: Option<i64>,
    ocr_pages: Option<i64>,
    file_bytes: Option<f64>,
    database_bytes: Option<f64>,
    archived_companies: Option<i64>,
    estimated_today_cents: Option<i64>,
}

impl PostV1BillingAccountGetResponseMonthToDateBuilder {
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn api_requests(mut self, value: i64) -> Self {
        self.api_requests = Some(value);
        self
    }

    pub fn ocr_pages(mut self, value: i64) -> Self {
        self.ocr_pages = Some(value);
        self
    }

    pub fn file_bytes(mut self, value: f64) -> Self {
        self.file_bytes = Some(value);
        self
    }

    pub fn database_bytes(mut self, value: f64) -> Self {
        self.database_bytes = Some(value);
        self
    }

    pub fn archived_companies(mut self, value: i64) -> Self {
        self.archived_companies = Some(value);
        self
    }

    pub fn estimated_today_cents(mut self, value: i64) -> Self {
        self.estimated_today_cents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingAccountGetResponseMonthToDate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](PostV1BillingAccountGetResponseMonthToDateBuilder::from)
    /// - [`to`](PostV1BillingAccountGetResponseMonthToDateBuilder::to)
    /// - [`api_requests`](PostV1BillingAccountGetResponseMonthToDateBuilder::api_requests)
    /// - [`ocr_pages`](PostV1BillingAccountGetResponseMonthToDateBuilder::ocr_pages)
    /// - [`file_bytes`](PostV1BillingAccountGetResponseMonthToDateBuilder::file_bytes)
    /// - [`database_bytes`](PostV1BillingAccountGetResponseMonthToDateBuilder::database_bytes)
    /// - [`archived_companies`](PostV1BillingAccountGetResponseMonthToDateBuilder::archived_companies)
    /// - [`estimated_today_cents`](PostV1BillingAccountGetResponseMonthToDateBuilder::estimated_today_cents)
    pub fn build(self) -> Result<PostV1BillingAccountGetResponseMonthToDate, BuildError> {
        Ok(PostV1BillingAccountGetResponseMonthToDate {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            api_requests: self
                .api_requests
                .ok_or_else(|| BuildError::missing_field("api_requests"))?,
            ocr_pages: self
                .ocr_pages
                .ok_or_else(|| BuildError::missing_field("ocr_pages"))?,
            file_bytes: self
                .file_bytes
                .ok_or_else(|| BuildError::missing_field("file_bytes"))?,
            database_bytes: self
                .database_bytes
                .ok_or_else(|| BuildError::missing_field("database_bytes"))?,
            archived_companies: self
                .archived_companies
                .ok_or_else(|| BuildError::missing_field("archived_companies"))?,
            estimated_today_cents: self
                .estimated_today_cents
                .ok_or_else(|| BuildError::missing_field("estimated_today_cents"))?,
        })
    }
}
