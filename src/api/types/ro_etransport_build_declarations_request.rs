pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RoEtransportBuildDeclarationsRequest {
    #[serde(rename = "waybillId")]
    #[serde(default)]
    pub waybill_id: String,
}

impl RoEtransportBuildDeclarationsRequest {
    pub fn builder() -> RoEtransportBuildDeclarationsRequestBuilder {
        <RoEtransportBuildDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RoEtransportBuildDeclarationsRequestBuilder {
    waybill_id: Option<String>,
}

impl RoEtransportBuildDeclarationsRequestBuilder {
    pub fn waybill_id(mut self, value: impl Into<String>) -> Self {
        self.waybill_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RoEtransportBuildDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`waybill_id`](RoEtransportBuildDeclarationsRequestBuilder::waybill_id)
    pub fn build(self) -> Result<RoEtransportBuildDeclarationsRequest, BuildError> {
        Ok(RoEtransportBuildDeclarationsRequest {
            waybill_id: self
                .waybill_id
                .ok_or_else(|| BuildError::missing_field("waybill_id"))?,
        })
    }
}
