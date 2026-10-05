pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VatDetailReportsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<VatDetailReportsRequestSide>,
}

impl VatDetailReportsRequest {
    pub fn builder() -> VatDetailReportsRequestBuilder {
        <VatDetailReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VatDetailReportsRequestBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    side: Option<VatDetailReportsRequestSide>,
}

impl VatDetailReportsRequestBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn side(mut self, value: VatDetailReportsRequestSide) -> Self {
        self.side = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VatDetailReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](VatDetailReportsRequestBuilder::from_date)
    /// - [`to_date`](VatDetailReportsRequestBuilder::to_date)
    pub fn build(self) -> Result<VatDetailReportsRequest, BuildError> {
        Ok(VatDetailReportsRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            side: self.side,
        })
    }
}
