pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TwoFactorDisableAccountRequest {
    #[serde(default)]
    pub code: String,
}

impl TwoFactorDisableAccountRequest {
    pub fn builder() -> TwoFactorDisableAccountRequestBuilder {
        <TwoFactorDisableAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TwoFactorDisableAccountRequestBuilder {
    code: Option<String>,
}

impl TwoFactorDisableAccountRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TwoFactorDisableAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](TwoFactorDisableAccountRequestBuilder::code)
    pub fn build(self) -> Result<TwoFactorDisableAccountRequest, BuildError> {
        Ok(TwoFactorDisableAccountRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
        })
    }
}
