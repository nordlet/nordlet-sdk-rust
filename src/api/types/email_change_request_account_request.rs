pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmailChangeRequestAccountRequest {
    #[serde(rename = "newEmail")]
    #[serde(default)]
    pub new_email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<EmailChangeRequestAccountRequestLocale>,
}

impl EmailChangeRequestAccountRequest {
    pub fn builder() -> EmailChangeRequestAccountRequestBuilder {
        <EmailChangeRequestAccountRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmailChangeRequestAccountRequestBuilder {
    new_email: Option<String>,
    locale: Option<EmailChangeRequestAccountRequestLocale>,
}

impl EmailChangeRequestAccountRequestBuilder {
    pub fn new_email(mut self, value: impl Into<String>) -> Self {
        self.new_email = Some(value.into());
        self
    }

    pub fn locale(mut self, value: EmailChangeRequestAccountRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EmailChangeRequestAccountRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`new_email`](EmailChangeRequestAccountRequestBuilder::new_email)
    pub fn build(self) -> Result<EmailChangeRequestAccountRequest, BuildError> {
        Ok(EmailChangeRequestAccountRequest {
            new_email: self
                .new_email
                .ok_or_else(|| BuildError::missing_field("new_email"))?,
            locale: self.locale,
        })
    }
}
