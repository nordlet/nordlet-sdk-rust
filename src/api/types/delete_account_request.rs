pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAccountRequest {
    #[serde(rename = "confirmEmail")]
    #[serde(default)]
    pub confirm_email: String,
}

impl DeleteAccountRequest {
    pub fn builder() -> DeleteAccountRequestBuilder {
        <DeleteAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAccountRequestBuilder {
    confirm_email: Option<String>,
}

impl DeleteAccountRequestBuilder {
    pub fn confirm_email(mut self, value: impl Into<String>) -> Self {
        self.confirm_email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`confirm_email`](DeleteAccountRequestBuilder::confirm_email)
    pub fn build(self) -> Result<DeleteAccountRequest, BuildError> {
        Ok(DeleteAccountRequest {
            confirm_email: self
                .confirm_email
                .ok_or_else(|| BuildError::missing_field("confirm_email"))?,
        })
    }
}
