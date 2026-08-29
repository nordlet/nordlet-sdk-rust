pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BillingUsageListRequest {
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
}

impl PostV1BillingUsageListRequest {
    pub fn builder() -> PostV1BillingUsageListRequestBuilder {
        <PostV1BillingUsageListRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BillingUsageListRequestBuilder {
    from: Option<String>,
    to: Option<String>,
}

impl PostV1BillingUsageListRequestBuilder {
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BillingUsageListRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](PostV1BillingUsageListRequestBuilder::from)
    /// - [`to`](PostV1BillingUsageListRequestBuilder::to)
    pub fn build(self) -> Result<PostV1BillingUsageListRequest, BuildError> {
        Ok(PostV1BillingUsageListRequest {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
