pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsGlDetailRequest {
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
}

impl PostV1ReportsGlDetailRequest {
    pub fn builder() -> PostV1ReportsGlDetailRequestBuilder {
        <PostV1ReportsGlDetailRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsGlDetailRequestBuilder {
    account_code: Option<String>,
    from_date: Option<String>,
    to_date: Option<String>,
}

impl PostV1ReportsGlDetailRequestBuilder {
    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsGlDetailRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_code`](PostV1ReportsGlDetailRequestBuilder::account_code)
    /// - [`from_date`](PostV1ReportsGlDetailRequestBuilder::from_date)
    /// - [`to_date`](PostV1ReportsGlDetailRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1ReportsGlDetailRequest, BuildError> {
        Ok(PostV1ReportsGlDetailRequest {
            account_code: self
                .account_code
                .ok_or_else(|| BuildError::missing_field("account_code"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
        })
    }
}
