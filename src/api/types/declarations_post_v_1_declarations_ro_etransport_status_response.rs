pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsRoEtransportStatusResponse {
    #[serde(default)]
    pub reference: String,
    pub state: PostV1DeclarationsRoEtransportStatusResponseState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl PostV1DeclarationsRoEtransportStatusResponse {
    pub fn builder() -> PostV1DeclarationsRoEtransportStatusResponseBuilder {
        <PostV1DeclarationsRoEtransportStatusResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsRoEtransportStatusResponseBuilder {
    reference: Option<String>,
    state: Option<PostV1DeclarationsRoEtransportStatusResponseState>,
    uit: Option<String>,
    detail: Option<String>,
}

impl PostV1DeclarationsRoEtransportStatusResponseBuilder {
    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn state(mut self, value: PostV1DeclarationsRoEtransportStatusResponseState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn uit(mut self, value: impl Into<String>) -> Self {
        self.uit = Some(value.into());
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsRoEtransportStatusResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference`](PostV1DeclarationsRoEtransportStatusResponseBuilder::reference)
    /// - [`state`](PostV1DeclarationsRoEtransportStatusResponseBuilder::state)
    pub fn build(self) -> Result<PostV1DeclarationsRoEtransportStatusResponse, BuildError> {
        Ok(PostV1DeclarationsRoEtransportStatusResponse {
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            uit: self.uit,
            detail: self.detail,
        })
    }
}
