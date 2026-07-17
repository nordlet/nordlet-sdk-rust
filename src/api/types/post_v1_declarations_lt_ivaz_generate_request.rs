pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIvazGenerateRequest {
    #[serde(rename = "waybillIds")]
    #[serde(default)]
    pub waybill_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist: Option<bool>,
}

impl PostV1DeclarationsLtIvazGenerateRequest {
    pub fn builder() -> PostV1DeclarationsLtIvazGenerateRequestBuilder {
        <PostV1DeclarationsLtIvazGenerateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIvazGenerateRequestBuilder {
    waybill_ids: Option<Vec<String>>,
    persist: Option<bool>,
}

impl PostV1DeclarationsLtIvazGenerateRequestBuilder {
    pub fn waybill_ids(mut self, value: Vec<String>) -> Self {
        self.waybill_ids = Some(value);
        self
    }

    pub fn persist(mut self, value: bool) -> Self {
        self.persist = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIvazGenerateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`waybill_ids`](PostV1DeclarationsLtIvazGenerateRequestBuilder::waybill_ids)
    pub fn build(self) -> Result<PostV1DeclarationsLtIvazGenerateRequest, BuildError> {
        Ok(PostV1DeclarationsLtIvazGenerateRequest {
            waybill_ids: self
                .waybill_ids
                .ok_or_else(|| BuildError::missing_field("waybill_ids"))?,
            persist: self.persist,
        })
    }
}
