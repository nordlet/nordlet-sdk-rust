pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsDeWithholdingGetDeclarationsRequest {
    #[serde(default)]
    pub period: String,
}

impl UsDeWithholdingGetDeclarationsRequest {
    pub fn builder() -> UsDeWithholdingGetDeclarationsRequestBuilder {
        <UsDeWithholdingGetDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsDeWithholdingGetDeclarationsRequestBuilder {
    period: Option<String>,
}

impl UsDeWithholdingGetDeclarationsRequestBuilder {
    pub fn period(mut self, value: impl Into<String>) -> Self {
        self.period = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsDeWithholdingGetDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`period`](UsDeWithholdingGetDeclarationsRequestBuilder::period)
    pub fn build(self) -> Result<UsDeWithholdingGetDeclarationsRequest, BuildError> {
        Ok(UsDeWithholdingGetDeclarationsRequest {
            period: self
                .period
                .ok_or_else(|| BuildError::missing_field("period"))?,
        })
    }
}
