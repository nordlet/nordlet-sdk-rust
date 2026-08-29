pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProjectsTimeEntriesBillResponse {
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: String,
    #[serde(rename = "entryCount")]
    #[serde(default)]
    pub entry_count: i64,
    #[serde(default)]
    pub hours: String,
    #[serde(rename = "netTotal")]
    #[serde(default)]
    pub net_total: String,
    #[serde(rename = "vatTotal")]
    #[serde(default)]
    pub vat_total: String,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
}

impl PostV1ProjectsTimeEntriesBillResponse {
    pub fn builder() -> PostV1ProjectsTimeEntriesBillResponseBuilder {
        <PostV1ProjectsTimeEntriesBillResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProjectsTimeEntriesBillResponseBuilder {
    invoice_id: Option<String>,
    entry_count: Option<i64>,
    hours: Option<String>,
    net_total: Option<String>,
    vat_total: Option<String>,
    gross_total: Option<String>,
}

impl PostV1ProjectsTimeEntriesBillResponseBuilder {
    pub fn invoice_id(mut self, value: impl Into<String>) -> Self {
        self.invoice_id = Some(value.into());
        self
    }

    pub fn entry_count(mut self, value: i64) -> Self {
        self.entry_count = Some(value);
        self
    }

    pub fn hours(mut self, value: impl Into<String>) -> Self {
        self.hours = Some(value.into());
        self
    }

    pub fn net_total(mut self, value: impl Into<String>) -> Self {
        self.net_total = Some(value.into());
        self
    }

    pub fn vat_total(mut self, value: impl Into<String>) -> Self {
        self.vat_total = Some(value.into());
        self
    }

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProjectsTimeEntriesBillResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](PostV1ProjectsTimeEntriesBillResponseBuilder::invoice_id)
    /// - [`entry_count`](PostV1ProjectsTimeEntriesBillResponseBuilder::entry_count)
    /// - [`hours`](PostV1ProjectsTimeEntriesBillResponseBuilder::hours)
    /// - [`net_total`](PostV1ProjectsTimeEntriesBillResponseBuilder::net_total)
    /// - [`vat_total`](PostV1ProjectsTimeEntriesBillResponseBuilder::vat_total)
    /// - [`gross_total`](PostV1ProjectsTimeEntriesBillResponseBuilder::gross_total)
    pub fn build(self) -> Result<PostV1ProjectsTimeEntriesBillResponse, BuildError> {
        Ok(PostV1ProjectsTimeEntriesBillResponse {
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            entry_count: self
                .entry_count
                .ok_or_else(|| BuildError::missing_field("entry_count"))?,
            hours: self
                .hours
                .ok_or_else(|| BuildError::missing_field("hours"))?,
            net_total: self
                .net_total
                .ok_or_else(|| BuildError::missing_field("net_total"))?,
            vat_total: self
                .vat_total
                .ok_or_else(|| BuildError::missing_field("vat_total"))?,
            gross_total: self
                .gross_total
                .ok_or_else(|| BuildError::missing_field("gross_total"))?,
        })
    }
}
