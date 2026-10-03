pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsRoEtransportSubmitResponse {
    #[serde(rename = "waybillId")]
    #[serde(default)]
    pub waybill_id: String,
    #[serde(default)]
    pub reference: String,
    pub state: PostV1DeclarationsRoEtransportSubmitResponseState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1DeclarationsRoEtransportSubmitResponse {
    pub fn builder() -> PostV1DeclarationsRoEtransportSubmitResponseBuilder {
        <PostV1DeclarationsRoEtransportSubmitResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsRoEtransportSubmitResponseBuilder {
    waybill_id: Option<String>,
    reference: Option<String>,
    state: Option<PostV1DeclarationsRoEtransportSubmitResponseState>,
    uit: Option<String>,
    detail: Option<String>,
    warnings: Option<Vec<String>>,
}

impl PostV1DeclarationsRoEtransportSubmitResponseBuilder {
    pub fn waybill_id(mut self, value: impl Into<String>) -> Self {
        self.waybill_id = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn state(mut self, value: PostV1DeclarationsRoEtransportSubmitResponseState) -> Self {
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

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsRoEtransportSubmitResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`waybill_id`](PostV1DeclarationsRoEtransportSubmitResponseBuilder::waybill_id)
    /// - [`reference`](PostV1DeclarationsRoEtransportSubmitResponseBuilder::reference)
    /// - [`state`](PostV1DeclarationsRoEtransportSubmitResponseBuilder::state)
    /// - [`warnings`](PostV1DeclarationsRoEtransportSubmitResponseBuilder::warnings)
    pub fn build(self) -> Result<PostV1DeclarationsRoEtransportSubmitResponse, BuildError> {
        Ok(PostV1DeclarationsRoEtransportSubmitResponse {
            waybill_id: self
                .waybill_id
                .ok_or_else(|| BuildError::missing_field("waybill_id"))?,
            reference: self
                .reference
                .ok_or_else(|| BuildError::missing_field("reference"))?,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            uit: self.uit,
            detail: self.detail,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
