pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwoFactorGetAccountResponse {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub required: bool,
    #[serde(rename = "recoveryCodesLeft")]
    #[serde(default)]
    pub recovery_codes_left: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup: Option<TwoFactorGetAccountResponseSetup>,
}

impl TwoFactorGetAccountResponse {
    pub fn builder() -> TwoFactorGetAccountResponseBuilder {
        <TwoFactorGetAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwoFactorGetAccountResponseBuilder {
    enabled: Option<bool>,
    required: Option<bool>,
    recovery_codes_left: Option<i64>,
    setup: Option<TwoFactorGetAccountResponseSetup>,
}

impl TwoFactorGetAccountResponseBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn recovery_codes_left(mut self, value: i64) -> Self {
        self.recovery_codes_left = Some(value);
        self
    }

    pub fn setup(mut self, value: TwoFactorGetAccountResponseSetup) -> Self {
        self.setup = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TwoFactorGetAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](TwoFactorGetAccountResponseBuilder::enabled)
    /// - [`required`](TwoFactorGetAccountResponseBuilder::required)
    /// - [`recovery_codes_left`](TwoFactorGetAccountResponseBuilder::recovery_codes_left)
    pub fn build(self) -> Result<TwoFactorGetAccountResponse, BuildError> {
        Ok(TwoFactorGetAccountResponse {
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            required: self
                .required
                .ok_or_else(|| BuildError::missing_field("required"))?,
            recovery_codes_left: self
                .recovery_codes_left
                .ok_or_else(|| BuildError::missing_field("recovery_codes_left"))?,
            setup: self.setup,
        })
    }
}
