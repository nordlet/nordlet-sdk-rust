pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwoFactorEnableAccountRequest {
    #[serde(default)]
    pub code: String,
}

impl TwoFactorEnableAccountRequest {
    pub fn builder() -> TwoFactorEnableAccountRequestBuilder {
        <TwoFactorEnableAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwoFactorEnableAccountRequestBuilder {
    code: Option<String>,
}

impl TwoFactorEnableAccountRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TwoFactorEnableAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](TwoFactorEnableAccountRequestBuilder::code)
    pub fn build(self) -> Result<TwoFactorEnableAccountRequest, BuildError> {
        Ok(TwoFactorEnableAccountRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
        })
    }
}
