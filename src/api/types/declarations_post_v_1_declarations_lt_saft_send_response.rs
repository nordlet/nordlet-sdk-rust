pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtSaftSendResponse {
    #[serde(rename = "caseId")]
    #[serde(default)]
    pub case_id: String,
    pub state: PostV1DeclarationsLtSaftSendResponseState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub confirmed: bool,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1DeclarationsLtSaftSendResponse {
    pub fn builder() -> PostV1DeclarationsLtSaftSendResponseBuilder {
        <PostV1DeclarationsLtSaftSendResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtSaftSendResponseBuilder {
    case_id: Option<String>,
    state: Option<PostV1DeclarationsLtSaftSendResponseState>,
    detail: Option<String>,
    file_name: Option<String>,
    confirmed: Option<bool>,
    warnings: Option<Vec<String>>,
}

impl PostV1DeclarationsLtSaftSendResponseBuilder {
    pub fn case_id(mut self, value: impl Into<String>) -> Self {
        self.case_id = Some(value.into());
        self
    }

    pub fn state(mut self, value: PostV1DeclarationsLtSaftSendResponseState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn confirmed(mut self, value: bool) -> Self {
        self.confirmed = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtSaftSendResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`case_id`](PostV1DeclarationsLtSaftSendResponseBuilder::case_id)
    /// - [`state`](PostV1DeclarationsLtSaftSendResponseBuilder::state)
    /// - [`file_name`](PostV1DeclarationsLtSaftSendResponseBuilder::file_name)
    /// - [`confirmed`](PostV1DeclarationsLtSaftSendResponseBuilder::confirmed)
    /// - [`warnings`](PostV1DeclarationsLtSaftSendResponseBuilder::warnings)
    pub fn build(self) -> Result<PostV1DeclarationsLtSaftSendResponse, BuildError> {
        Ok(PostV1DeclarationsLtSaftSendResponse {
            case_id: self
                .case_id
                .ok_or_else(|| BuildError::missing_field("case_id"))?,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            detail: self.detail,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            confirmed: self
                .confirmed
                .ok_or_else(|| BuildError::missing_field("confirmed"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
