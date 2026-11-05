pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwoFactorGetAccountResponseSetup {
    #[serde(default)]
    pub secret: String,
    #[serde(rename = "otpauthUrl")]
    #[serde(default)]
    pub otpauth_url: String,
}

impl TwoFactorGetAccountResponseSetup {
    pub fn builder() -> TwoFactorGetAccountResponseSetupBuilder {
        <TwoFactorGetAccountResponseSetupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwoFactorGetAccountResponseSetupBuilder {
    secret: Option<String>,
    otpauth_url: Option<String>,
}

impl TwoFactorGetAccountResponseSetupBuilder {
    pub fn secret(mut self, value: impl Into<String>) -> Self {
        self.secret = Some(value.into());
        self
    }

    pub fn otpauth_url(mut self, value: impl Into<String>) -> Self {
        self.otpauth_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TwoFactorGetAccountResponseSetup`].
    /// This method will fail if any of the following fields are not set:
    /// - [`secret`](TwoFactorGetAccountResponseSetupBuilder::secret)
    /// - [`otpauth_url`](TwoFactorGetAccountResponseSetupBuilder::otpauth_url)
    pub fn build(self) -> Result<TwoFactorGetAccountResponseSetup, BuildError> {
        Ok(TwoFactorGetAccountResponseSetup {
            secret: self
                .secret
                .ok_or_else(|| BuildError::missing_field("secret"))?,
            otpauth_url: self
                .otpauth_url
                .ok_or_else(|| BuildError::missing_field("otpauth_url"))?,
        })
    }
}
