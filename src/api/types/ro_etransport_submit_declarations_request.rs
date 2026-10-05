pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RoEtransportSubmitDeclarationsRequest {
    #[serde(rename = "waybillId")]
    #[serde(default)]
    pub waybill_id: String,
}

impl RoEtransportSubmitDeclarationsRequest {
    pub fn builder() -> RoEtransportSubmitDeclarationsRequestBuilder {
        <RoEtransportSubmitDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RoEtransportSubmitDeclarationsRequestBuilder {
    waybill_id: Option<String>,
}

impl RoEtransportSubmitDeclarationsRequestBuilder {
    pub fn waybill_id(mut self, value: impl Into<String>) -> Self {
        self.waybill_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RoEtransportSubmitDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`waybill_id`](RoEtransportSubmitDeclarationsRequestBuilder::waybill_id)
    pub fn build(self) -> Result<RoEtransportSubmitDeclarationsRequest, BuildError> {
        Ok(RoEtransportSubmitDeclarationsRequest {
            waybill_id: self
                .waybill_id
                .ok_or_else(|| BuildError::missing_field("waybill_id"))?,
        })
    }
}
