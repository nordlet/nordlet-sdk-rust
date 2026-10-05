pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WriteOffActsReportsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl WriteOffActsReportsRequest {
    pub fn builder() -> WriteOffActsReportsRequestBuilder {
        <WriteOffActsReportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WriteOffActsReportsRequestBuilder {
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    warehouse_id: Option<String>,
}

impl WriteOffActsReportsRequestBuilder {
    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WriteOffActsReportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](WriteOffActsReportsRequestBuilder::from_date)
    /// - [`to_date`](WriteOffActsReportsRequestBuilder::to_date)
    pub fn build(self) -> Result<WriteOffActsReportsRequest, BuildError> {
        Ok(WriteOffActsReportsRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            warehouse_id: self.warehouse_id,
        })
    }
}
