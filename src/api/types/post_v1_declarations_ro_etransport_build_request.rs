pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsRoEtransportBuildRequest {
    #[serde(rename = "waybillId")]
    #[serde(default)]
    pub waybill_id: String,
}

impl PostV1DeclarationsRoEtransportBuildRequest {
    pub fn builder() -> PostV1DeclarationsRoEtransportBuildRequestBuilder {
        <PostV1DeclarationsRoEtransportBuildRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsRoEtransportBuildRequestBuilder {
    waybill_id: Option<String>,
}

impl PostV1DeclarationsRoEtransportBuildRequestBuilder {
    pub fn waybill_id(mut self, value: impl Into<String>) -> Self {
        self.waybill_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsRoEtransportBuildRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`waybill_id`](PostV1DeclarationsRoEtransportBuildRequestBuilder::waybill_id)
    pub fn build(self) -> Result<PostV1DeclarationsRoEtransportBuildRequest, BuildError> {
        Ok(PostV1DeclarationsRoEtransportBuildRequest {
            waybill_id: self
                .waybill_id
                .ok_or_else(|| BuildError::missing_field("waybill_id"))?,
        })
    }
}
