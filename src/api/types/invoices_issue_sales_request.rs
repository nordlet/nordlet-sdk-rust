pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesIssueSalesRequest {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[serde(rename = "issueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<NaiveDate>,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl InvoicesIssueSalesRequest {
    pub fn builder() -> InvoicesIssueSalesRequestBuilder {
        <InvoicesIssueSalesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesIssueSalesRequestBuilder {
    id: Option<String>,
    series: Option<String>,
    issue_date: Option<NaiveDate>,
    warehouse_id: Option<String>,
}

impl InvoicesIssueSalesRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn series(mut self, value: impl Into<String>) -> Self {
        self.series = Some(value.into());
        self
    }

    pub fn issue_date(mut self, value: NaiveDate) -> Self {
        self.issue_date = Some(value);
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicesIssueSalesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicesIssueSalesRequestBuilder::id)
    pub fn build(self) -> Result<InvoicesIssueSalesRequest, BuildError> {
        Ok(InvoicesIssueSalesRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            series: self.series,
            issue_date: self.issue_date,
            warehouse_id: self.warehouse_id,
        })
    }
}
