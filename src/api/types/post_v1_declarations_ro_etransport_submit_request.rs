pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsRoEtransportSubmitRequest {
    #[serde(rename = "waybillId")]
    #[serde(default)]
    pub waybill_id: String,
}

impl PostV1DeclarationsRoEtransportSubmitRequest {
    pub fn builder() -> PostV1DeclarationsRoEtransportSubmitRequestBuilder {
        <PostV1DeclarationsRoEtransportSubmitRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsRoEtransportSubmitRequestBuilder {
    waybill_id: Option<String>,
}

impl PostV1DeclarationsRoEtransportSubmitRequestBuilder {
    pub fn waybill_id(mut self, value: impl Into<String>) -> Self {
        self.waybill_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsRoEtransportSubmitRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`waybill_id`](PostV1DeclarationsRoEtransportSubmitRequestBuilder::waybill_id)
    pub fn build(self) -> Result<PostV1DeclarationsRoEtransportSubmitRequest, BuildError> {
        Ok(PostV1DeclarationsRoEtransportSubmitRequest {
            waybill_id: self
                .waybill_id
                .ok_or_else(|| BuildError::missing_field("waybill_id"))?,
        })
    }
}
