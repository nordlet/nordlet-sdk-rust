pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LogoutAccountResponse {
    #[serde(rename = "loggedOut")]
    #[serde(default)]
    pub logged_out: bool,
}

impl LogoutAccountResponse {
    pub fn builder() -> LogoutAccountResponseBuilder {
        <LogoutAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LogoutAccountResponseBuilder {
    logged_out: Option<bool>,
}

impl LogoutAccountResponseBuilder {
    pub fn logged_out(mut self, value: bool) -> Self {
        self.logged_out = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LogoutAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`logged_out`](LogoutAccountResponseBuilder::logged_out)
    pub fn build(self) -> Result<LogoutAccountResponse, BuildError> {
        Ok(LogoutAccountResponse {
            logged_out: self
                .logged_out
                .ok_or_else(|| BuildError::missing_field("logged_out"))?,
        })
    }
}
