pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwoFactorRecoveryCodesRegenerateAccountRequest {
    #[serde(default)]
    pub code: String,
}

impl TwoFactorRecoveryCodesRegenerateAccountRequest {
    pub fn builder() -> TwoFactorRecoveryCodesRegenerateAccountRequestBuilder {
        <TwoFactorRecoveryCodesRegenerateAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwoFactorRecoveryCodesRegenerateAccountRequestBuilder {
    code: Option<String>,
}

impl TwoFactorRecoveryCodesRegenerateAccountRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TwoFactorRecoveryCodesRegenerateAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](TwoFactorRecoveryCodesRegenerateAccountRequestBuilder::code)
    pub fn build(self) -> Result<TwoFactorRecoveryCodesRegenerateAccountRequest, BuildError> {
        Ok(TwoFactorRecoveryCodesRegenerateAccountRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
        })
    }
}
