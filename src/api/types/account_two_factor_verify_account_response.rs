pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwoFactorVerifyAccountResponse {
    #[serde(default)]
    pub verified: bool,
}

impl TwoFactorVerifyAccountResponse {
    pub fn builder() -> TwoFactorVerifyAccountResponseBuilder {
        <TwoFactorVerifyAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwoFactorVerifyAccountResponseBuilder {
    verified: Option<bool>,
}

impl TwoFactorVerifyAccountResponseBuilder {
    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TwoFactorVerifyAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`verified`](TwoFactorVerifyAccountResponseBuilder::verified)
    pub fn build(self) -> Result<TwoFactorVerifyAccountResponse, BuildError> {
        Ok(TwoFactorVerifyAccountResponse {
            verified: self
                .verified
                .ok_or_else(|| BuildError::missing_field("verified"))?,
        })
    }
}
