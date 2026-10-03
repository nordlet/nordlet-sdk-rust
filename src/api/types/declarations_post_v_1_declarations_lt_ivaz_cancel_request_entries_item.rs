pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIvazCancelRequestEntriesItem {
    #[serde(rename = "waybillId")]
    #[serde(default)]
    pub waybill_id: String,
    pub reason: PostV1DeclarationsLtIvazCancelRequestEntriesItemReason,
    #[serde(rename = "additionalInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl PostV1DeclarationsLtIvazCancelRequestEntriesItem {
    pub fn builder() -> PostV1DeclarationsLtIvazCancelRequestEntriesItemBuilder {
        <PostV1DeclarationsLtIvazCancelRequestEntriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIvazCancelRequestEntriesItemBuilder {
    waybill_id: Option<String>,
    reason: Option<PostV1DeclarationsLtIvazCancelRequestEntriesItemReason>,
    additional_info: Option<String>,
}

impl PostV1DeclarationsLtIvazCancelRequestEntriesItemBuilder {
    pub fn waybill_id(mut self, value: impl Into<String>) -> Self {
        self.waybill_id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: PostV1DeclarationsLtIvazCancelRequestEntriesItemReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn additional_info(mut self, value: impl Into<String>) -> Self {
        self.additional_info = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIvazCancelRequestEntriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`waybill_id`](PostV1DeclarationsLtIvazCancelRequestEntriesItemBuilder::waybill_id)
    /// - [`reason`](PostV1DeclarationsLtIvazCancelRequestEntriesItemBuilder::reason)
    pub fn build(self) -> Result<PostV1DeclarationsLtIvazCancelRequestEntriesItem, BuildError> {
        Ok(PostV1DeclarationsLtIvazCancelRequestEntriesItem {
            waybill_id: self
                .waybill_id
                .ok_or_else(|| BuildError::missing_field("waybill_id"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            additional_info: self.additional_info,
        })
    }
}
