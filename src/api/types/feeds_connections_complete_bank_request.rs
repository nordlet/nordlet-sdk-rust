pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedsConnectionsCompleteBankRequest {
    #[serde(default)]
    pub reference: String,
    #[serde(default)]
    pub code: String,
}

impl FeedsConnectionsCompleteBankRequest {
    pub fn builder() -> FeedsConnectionsCompleteBankRequestBuilder {
        <FeedsConnectionsCompleteBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsConnectionsCompleteBankRequestBuilder {
    reference: Option<String>,
    code: Option<String>,
}

impl FeedsConnectionsCompleteBankRequestBuilder {
    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FeedsConnectionsCompleteBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference`](FeedsConnectionsCompleteBankRequestBuilder::reference)
    /// - [`code`](FeedsConnectionsCompleteBankRequestBuilder::code)
    pub fn build(self) -> Result<FeedsConnectionsCompleteBankRequest, BuildError> {
        Ok(FeedsConnectionsCompleteBankRequest {
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
        })
    }
}
