pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsWriteOffActsRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(rename = "warehouseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warehouse_id: Option<String>,
}

impl PostV1ReportsWriteOffActsRequest {
    pub fn builder() -> PostV1ReportsWriteOffActsRequestBuilder {
        <PostV1ReportsWriteOffActsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsWriteOffActsRequestBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    warehouse_id: Option<String>,
}

impl PostV1ReportsWriteOffActsRequestBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn warehouse_id(mut self, value: impl Into<String>) -> Self {
        self.warehouse_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsWriteOffActsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsWriteOffActsRequestBuilder::from_date)
    /// - [`to_date`](PostV1ReportsWriteOffActsRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1ReportsWriteOffActsRequest, BuildError> {
        Ok(PostV1ReportsWriteOffActsRequest {
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
