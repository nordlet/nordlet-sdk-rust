pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwoFactorDisableAccountResponse {
    #[serde(default)]
    pub enabled: bool,
}

impl TwoFactorDisableAccountResponse {
    pub fn builder() -> TwoFactorDisableAccountResponseBuilder {
        <TwoFactorDisableAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwoFactorDisableAccountResponseBuilder {
    enabled: Option<bool>,
}

impl TwoFactorDisableAccountResponseBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TwoFactorDisableAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](TwoFactorDisableAccountResponseBuilder::enabled)
    pub fn build(self) -> Result<TwoFactorDisableAccountResponse, BuildError> {
        Ok(TwoFactorDisableAccountResponse {
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
        })
    }
}
