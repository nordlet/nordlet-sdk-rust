pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwoFactorRecoveryCodesRegenerateAccountResponse {
    #[serde(rename = "recoveryCodes")]
    #[serde(default)]
    pub recovery_codes: Vec<String>,
}

impl TwoFactorRecoveryCodesRegenerateAccountResponse {
    pub fn builder() -> TwoFactorRecoveryCodesRegenerateAccountResponseBuilder {
        <TwoFactorRecoveryCodesRegenerateAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwoFactorRecoveryCodesRegenerateAccountResponseBuilder {
    recovery_codes: Option<Vec<String>>,
}

impl TwoFactorRecoveryCodesRegenerateAccountResponseBuilder {
    pub fn recovery_codes(mut self, value: Vec<String>) -> Self {
        self.recovery_codes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TwoFactorRecoveryCodesRegenerateAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recovery_codes`](TwoFactorRecoveryCodesRegenerateAccountResponseBuilder::recovery_codes)
    pub fn build(self) -> Result<TwoFactorRecoveryCodesRegenerateAccountResponse, BuildError> {
        Ok(TwoFactorRecoveryCodesRegenerateAccountResponse {
            recovery_codes: self
                .recovery_codes
                .ok_or_else(|| BuildError::missing_field("recovery_codes"))?,
        })
    }
}
