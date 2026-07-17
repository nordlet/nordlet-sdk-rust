pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsDebtAgingRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<PostV1ReportsDebtAgingRequestSide>,
    #[serde(rename = "asOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of: Option<String>,
}

impl PostV1ReportsDebtAgingRequest {
    pub fn builder() -> PostV1ReportsDebtAgingRequestBuilder {
        <PostV1ReportsDebtAgingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsDebtAgingRequestBuilder {
    side: Option<PostV1ReportsDebtAgingRequestSide>,
    as_of: Option<String>,
}

impl PostV1ReportsDebtAgingRequestBuilder {
    pub fn side(mut self, value: PostV1ReportsDebtAgingRequestSide) -> Self {
        self.side = Some(value);
        self
    }

    pub fn as_of(mut self, value: impl Into<String>) -> Self {
        self.as_of = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReportsDebtAgingRequest`].
    pub fn build(self) -> Result<PostV1ReportsDebtAgingRequest, BuildError> {
        Ok(PostV1ReportsDebtAgingRequest {
            side: self.side,
            as_of: self.as_of,
        })
    }
}
