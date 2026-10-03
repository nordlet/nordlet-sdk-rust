pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsDatevRequest {
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(rename = "consultantNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consultant_number: Option<String>,
    #[serde(rename = "clientNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_number: Option<String>,
}

impl PostV1ReportsDatevRequest {
    pub fn builder() -> PostV1ReportsDatevRequestBuilder {
        <PostV1ReportsDatevRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsDatevRequestBuilder {
    from_date: Option<String>,
    to_date: Option<String>,
    consultant_number: Option<String>,
    client_number: Option<String>,
}

impl PostV1ReportsDatevRequestBuilder {
    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostV1ReportsDatevRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_date`](PostV1ReportsDatevRequestBuilder::from_date)
    /// - [`to_date`](PostV1ReportsDatevRequestBuilder::to_date)
    pub fn build(self) -> Result<PostV1ReportsDatevRequest, BuildError> {
        Ok(PostV1ReportsDatevRequest {
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
