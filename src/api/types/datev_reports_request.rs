pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatevReportsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(rename = "consultantNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consultant_number: Option<String>,
    #[serde(rename = "clientNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_number: Option<String>,
}

impl DatevReportsRequest {
    pub fn builder() -> DatevReportsRequestBuilder {
        <DatevReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatevReportsRequestBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    consultant_number: Option<String>,
    client_number: Option<String>,
}

impl DatevReportsRequestBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn consultant_number(mut self, value: impl Into<String>) -> Self {
        self.consultant_number = Some(value.into());
        self
    }

    pub fn client_number(mut self, value: impl Into<String>) -> Self {
        self.client_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DatevReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](DatevReportsRequestBuilder::from_date)
    /// - [`to_date`](DatevReportsRequestBuilder::to_date)
    pub fn build(self) -> Result<DatevReportsRequest, BuildError> {
        Ok(DatevReportsRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            consultant_number: self.consultant_number,
            client_number: self.client_number,
        })
    }
}
