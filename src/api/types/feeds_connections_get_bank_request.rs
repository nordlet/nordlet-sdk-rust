pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedsConnectionsGetBankRequest {
    #[serde(default)]
    pub id: String,
}

impl FeedsConnectionsGetBankRequest {
    pub fn builder() -> FeedsConnectionsGetBankRequestBuilder {
        <FeedsConnectionsGetBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsConnectionsGetBankRequestBuilder {
    id: Option<String>,
}

impl FeedsConnectionsGetBankRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FeedsConnectionsGetBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](FeedsConnectionsGetBankRequestBuilder::id)
    pub fn build(self) -> Result<FeedsConnectionsGetBankRequest, BuildError> {
        Ok(FeedsConnectionsGetBankRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
