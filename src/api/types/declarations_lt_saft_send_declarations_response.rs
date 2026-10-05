pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LtSaftSendDeclarationsResponse {
    #[serde(rename = "submissionId")]
    #[serde(default)]
    pub submission_id: String,
    #[serde(rename = "caseId")]
    #[serde(default)]
    pub case_id: String,
    pub state: LtSaftSendDeclarationsResponseState,
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

impl LtSaftSendDeclarationsResponse {
    pub fn builder() -> LtSaftSendDeclarationsResponseBuilder {
        <LtSaftSendDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtSaftSendDeclarationsResponseBuilder {
    submission_id: Option<String>,
    case_id: Option<String>,
    state: Option<LtSaftSendDeclarationsResponseState>,
    detail: Option<String>,
    file_name: Option<String>,
    confirmed: Option<bool>,
    warnings: Option<Vec<String>>,
}

impl LtSaftSendDeclarationsResponseBuilder {
    pub fn submission_id(mut self, value: impl Into<String>) -> Self {
        self.submission_id = Some(value.into());
        self
    }

    pub fn case_id(mut self, value: impl Into<String>) -> Self {
        self.case_id = Some(value.into());
        self
    }

    pub fn state(mut self, value: LtSaftSendDeclarationsResponseState) -> Self {
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

    /// Consumes the builder and constructs a [`LtSaftSendDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`submission_id`](LtSaftSendDeclarationsResponseBuilder::submission_id)
    /// - [`case_id`](LtSaftSendDeclarationsResponseBuilder::case_id)
    /// - [`state`](LtSaftSendDeclarationsResponseBuilder::state)
    /// - [`file_name`](LtSaftSendDeclarationsResponseBuilder::file_name)
    /// - [`confirmed`](LtSaftSendDeclarationsResponseBuilder::confirmed)
    /// - [`warnings`](LtSaftSendDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<LtSaftSendDeclarationsResponse, BuildError> {
        Ok(LtSaftSendDeclarationsResponse {
            submission_id: self
                .submission_id
                .ok_or_else(|| BuildError::missing_field("submission_id"))?,
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
