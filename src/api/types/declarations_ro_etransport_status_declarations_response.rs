pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RoEtransportStatusDeclarationsResponse {
    #[serde(default)]
    pub reference: String,
    pub state: RoEtransportStatusDeclarationsResponseState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl RoEtransportStatusDeclarationsResponse {
    pub fn builder() -> RoEtransportStatusDeclarationsResponseBuilder {
        <RoEtransportStatusDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RoEtransportStatusDeclarationsResponseBuilder {
    reference: Option<String>,
    state: Option<RoEtransportStatusDeclarationsResponseState>,
    uit: Option<String>,
    detail: Option<String>,
}

impl RoEtransportStatusDeclarationsResponseBuilder {
    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn state(mut self, value: RoEtransportStatusDeclarationsResponseState) -> Self {
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

    /// Consumes the builder and constructs a [`RoEtransportStatusDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference`](RoEtransportStatusDeclarationsResponseBuilder::reference)
    /// - [`state`](RoEtransportStatusDeclarationsResponseBuilder::state)
    pub fn build(self) -> Result<RoEtransportStatusDeclarationsResponse, BuildError> {
        Ok(RoEtransportStatusDeclarationsResponse {
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
