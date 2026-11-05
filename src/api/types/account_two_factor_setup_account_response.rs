pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwoFactorSetupAccountResponse {
    #[serde(default)]
    pub secret: String,
    #[serde(rename = "otpauthUrl")]
    #[serde(default)]
    pub otpauth_url: String,
}

impl TwoFactorSetupAccountResponse {
    pub fn builder() -> TwoFactorSetupAccountResponseBuilder {
        <TwoFactorSetupAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwoFactorSetupAccountResponseBuilder {
    secret: Option<String>,
    otpauth_url: Option<String>,
}

impl TwoFactorSetupAccountResponseBuilder {
    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    pub fn otpauth_url(mut self, value: impl Into<String>) -> Self {
        self.otpauth_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TwoFactorSetupAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`secret`](TwoFactorSetupAccountResponseBuilder::secret)
    /// - [`otpauth_url`](TwoFactorSetupAccountResponseBuilder::otpauth_url)
    pub fn build(self) -> Result<TwoFactorSetupAccountResponse, BuildError> {
        Ok(TwoFactorSetupAccountResponse {
            secret: self
                .secret
                .ok_or_else(|| BuildError::missing_field("secret"))?,
            otpauth_url: self
                .otpauth_url
                .ok_or_else(|| BuildError::missing_field("otpauth_url"))?,
        })
    }
}
