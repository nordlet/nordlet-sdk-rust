pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwoFactorVerifyAccountRequest {
    #[serde(default)]
    pub code: String,
}

impl TwoFactorVerifyAccountRequest {
    pub fn builder() -> TwoFactorVerifyAccountRequestBuilder {
        <TwoFactorVerifyAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwoFactorVerifyAccountRequestBuilder {
    code: Option<String>,
}

impl TwoFactorVerifyAccountRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TwoFactorVerifyAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](TwoFactorVerifyAccountRequestBuilder::code)
    pub fn build(self) -> Result<TwoFactorVerifyAccountRequest, BuildError> {
        Ok(TwoFactorVerifyAccountRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
        })
    }
}
