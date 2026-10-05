pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RoEtransportSubmitDeclarationsResponse {
    #[serde(rename = "waybillId")]
    #[serde(default)]
    pub waybill_id: String,
    #[serde(default)]
    pub reference: String,
    pub state: RoEtransportSubmitDeclarationsResponseState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl RoEtransportSubmitDeclarationsResponse {
    pub fn builder() -> RoEtransportSubmitDeclarationsResponseBuilder {
        <RoEtransportSubmitDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RoEtransportSubmitDeclarationsResponseBuilder {
    waybill_id: Option<String>,
    reference: Option<String>,
    state: Option<RoEtransportSubmitDeclarationsResponseState>,
    uit: Option<String>,
    detail: Option<String>,
    warnings: Option<Vec<String>>,
}

impl RoEtransportSubmitDeclarationsResponseBuilder {
    pub fn waybill_id(mut self, value: impl Into<String>) -> Self {
        self.waybill_id = Some(value.into());
        self
    }

    pub fn reference(mut self, value: impl Into<String>) -> Self {
        self.reference = Some(value.into());
        self
    }

    pub fn state(mut self, value: RoEtransportSubmitDeclarationsResponseState) -> Self {
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

    /// Consumes the builder and constructs a [`RoEtransportSubmitDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`waybill_id`](RoEtransportSubmitDeclarationsResponseBuilder::waybill_id)
    /// - [`reference`](RoEtransportSubmitDeclarationsResponseBuilder::reference)
    /// - [`state`](RoEtransportSubmitDeclarationsResponseBuilder::state)
    /// - [`warnings`](RoEtransportSubmitDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<RoEtransportSubmitDeclarationsResponse, BuildError> {
        Ok(RoEtransportSubmitDeclarationsResponse {
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
