pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmailChangeRequestAccountResponse {
    #[serde(default)]
    pub sent: bool,
}

impl EmailChangeRequestAccountResponse {
    pub fn builder() -> EmailChangeRequestAccountResponseBuilder {
        <EmailChangeRequestAccountResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailChangeRequestAccountResponseBuilder {
    sent: Option<bool>,
}

impl EmailChangeRequestAccountResponseBuilder {
    pub fn sent(mut self, value: bool) -> Self {
        self.sent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmailChangeRequestAccountResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sent`](EmailChangeRequestAccountResponseBuilder::sent)
    pub fn build(self) -> Result<EmailChangeRequestAccountResponse, BuildError> {
        Ok(EmailChangeRequestAccountResponse {
            sent: self.sent.ok_or_else(|| BuildError::missing_field("sent"))?,
        })
    }
}
