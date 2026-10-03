pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsSieRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(rename = "includeTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transactions: Option<bool>,
}

impl PostV1ReportsSieRequest {
    pub fn builder() -> PostV1ReportsSieRequestBuilder {
        <PostV1ReportsSieRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsSieRequestBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    include_transactions: Option<bool>,
}

impl PostV1ReportsSieRequestBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn include_transactions(mut self, value: bool) -> Self {
        self.include_transactions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsSieRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsSieRequestBuilder::from_date)
    /// - [`to_date`](PostV1ReportsSieRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1ReportsSieRequest, BuildError> {
        Ok(PostV1ReportsSieRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            include_transactions: self.include_transactions,
        })
    }
}
