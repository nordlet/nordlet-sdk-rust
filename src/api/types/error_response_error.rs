pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorResponseError {
    pub code: ErrorResponseErrorCode,
    #[serde(default)]
    pub message: String,
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
    #[serde(rename = "fieldErrors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_errors: Option<HashMap<String, Vec<String>>>,
}

impl ErrorResponseError {
    pub fn builder() -> ErrorResponseErrorBuilder {
        <ErrorResponseErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorResponseErrorBuilder {
    code: Option<ErrorResponseErrorCode>,
    message: Option<String>,
    request_id: Option<String>,
    field_errors: Option<HashMap<String, Vec<String>>>,
}

impl ErrorResponseErrorBuilder {
    pub fn code(mut self, value: ErrorResponseErrorCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn field_errors(mut self, value: HashMap<String, Vec<String>>) -> Self {
        self.field_errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ErrorResponseError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ErrorResponseErrorBuilder::code)
    /// - [`message`](ErrorResponseErrorBuilder::message)
    /// - [`request_id`](ErrorResponseErrorBuilder::request_id)
    pub fn build(self) -> Result<ErrorResponseError, BuildError> {
        Ok(ErrorResponseError {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            request_id: self
                .request_id
                .ok_or_else(|| BuildError::missing_field("request_id"))?,
            field_errors: self.field_errors,
        })
    }
}
