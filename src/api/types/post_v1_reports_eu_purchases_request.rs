pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsEuPurchasesRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
}

impl PostV1ReportsEuPurchasesRequest {
    pub fn builder() -> PostV1ReportsEuPurchasesRequestBuilder {
        <PostV1ReportsEuPurchasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsEuPurchasesRequestBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
}

impl PostV1ReportsEuPurchasesRequestBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsEuPurchasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsEuPurchasesRequestBuilder::from_date)
    /// - [`to_date`](PostV1ReportsEuPurchasesRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1ReportsEuPurchasesRequest, BuildError> {
        Ok(PostV1ReportsEuPurchasesRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
        })
    }
}
