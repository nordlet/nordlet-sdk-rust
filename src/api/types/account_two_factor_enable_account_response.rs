pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwoFactorEnableAccountResponse {
    #[serde(rename = "recoveryCodes")]
    #[serde(default)]
    pub recovery_codes: Vec<String>,
}

impl TwoFactorEnableAccountResponse {
    pub fn builder() -> TwoFactorEnableAccountResponseBuilder {
        <TwoFactorEnableAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwoFactorEnableAccountResponseBuilder {
    recovery_codes: Option<Vec<String>>,
}

impl TwoFactorEnableAccountResponseBuilder {
    pub fn recovery_codes(mut self, value: Vec<String>) -> Self {
        self.recovery_codes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TwoFactorEnableAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recovery_codes`](TwoFactorEnableAccountResponseBuilder::recovery_codes)
    pub fn build(self) -> Result<TwoFactorEnableAccountResponse, BuildError> {
        Ok(TwoFactorEnableAccountResponse {
            recovery_codes: self
                .recovery_codes
                .ok_or_else(|| BuildError::missing_field("recovery_codes"))?,
        })
    }
}
