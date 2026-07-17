pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsVatDetailRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<PostV1ReportsVatDetailRequestSide>,
}

impl PostV1ReportsVatDetailRequest {
    pub fn builder() -> PostV1ReportsVatDetailRequestBuilder {
        <PostV1ReportsVatDetailRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsVatDetailRequestBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    side: Option<PostV1ReportsVatDetailRequestSide>,
}

impl PostV1ReportsVatDetailRequestBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn side(mut self, value: PostV1ReportsVatDetailRequestSide) -> Self {
        self.side = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsVatDetailRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsVatDetailRequestBuilder::from_date)
    /// - [`to_date`](PostV1ReportsVatDetailRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1ReportsVatDetailRequest, BuildError> {
        Ok(PostV1ReportsVatDetailRequest {
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
