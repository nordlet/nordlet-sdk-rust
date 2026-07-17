pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsCostCenterActivityRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(rename = "costCenterId")]
    #[serde(default)]
    pub cost_center_id: String,
}

impl PostV1ReportsCostCenterActivityRequest {
    pub fn builder() -> PostV1ReportsCostCenterActivityRequestBuilder {
        <PostV1ReportsCostCenterActivityRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsCostCenterActivityRequestBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    cost_center_id: Option<String>,
}

impl PostV1ReportsCostCenterActivityRequestBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn cost_center_id(mut self, value: impl Into<String>) -> Self {
        self.cost_center_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsCostCenterActivityRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsCostCenterActivityRequestBuilder::from_date)
    /// - [`to_date`](PostV1ReportsCostCenterActivityRequestBuilder::to_date)
    /// - [`cost_center_id`](PostV1ReportsCostCenterActivityRequestBuilder::cost_center_id)
    pub fn build(self) -> Result<PostV1ReportsCostCenterActivityRequest, BuildError> {
        Ok(PostV1ReportsCostCenterActivityRequest {
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            cost_center_id: self
                .cost_center_id
                .ok_or_else(|| BuildError::missing_field("cost_center_id"))?,
        })
    }
}
