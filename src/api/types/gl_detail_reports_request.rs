pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GlDetailReportsRequest {
    #[serde(rename = "accountCode")]
    #[serde(default)]
    pub account_code: String,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
}

impl GlDetailReportsRequest {
    pub fn builder() -> GlDetailReportsRequestBuilder {
        <GlDetailReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GlDetailReportsRequestBuilder {
    account_code: Option<String>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
}

impl GlDetailReportsRequestBuilder {
    pub fn account_code(mut self, value: impl Into<String>) -> Self {
        self.account_code = Some(value.into());
        self
    }

    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GlDetailReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_code`](GlDetailReportsRequestBuilder::account_code)
    /// - [`from_date`](GlDetailReportsRequestBuilder::from_date)
    /// - [`to_date`](GlDetailReportsRequestBuilder::to_date)
    pub fn build(self) -> Result<GlDetailReportsRequest, BuildError> {
        Ok(GlDetailReportsRequest {
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
