pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LtIvazCancelDeclarationsRequestEntriesItem {
    #[serde(rename = "waybillId")]
    #[serde(default)]
    pub waybill_id: String,
    pub reason: LtIvazCancelDeclarationsRequestEntriesItemReason,
    #[serde(rename = "additionalInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl LtIvazCancelDeclarationsRequestEntriesItem {
    pub fn builder() -> LtIvazCancelDeclarationsRequestEntriesItemBuilder {
        <LtIvazCancelDeclarationsRequestEntriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtIvazCancelDeclarationsRequestEntriesItemBuilder {
    waybill_id: Option<String>,
    reason: Option<LtIvazCancelDeclarationsRequestEntriesItemReason>,
    additional_info: Option<String>,
}

impl LtIvazCancelDeclarationsRequestEntriesItemBuilder {
    pub fn waybill_id(mut self, value: impl Into<String>) -> Self {
        self.waybill_id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: LtIvazCancelDeclarationsRequestEntriesItemReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn additional_info(mut self, value: impl Into<String>) -> Self {
        self.additional_info = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LtIvazCancelDeclarationsRequestEntriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`waybill_id`](LtIvazCancelDeclarationsRequestEntriesItemBuilder::waybill_id)
    /// - [`reason`](LtIvazCancelDeclarationsRequestEntriesItemBuilder::reason)
    pub fn build(self) -> Result<LtIvazCancelDeclarationsRequestEntriesItem, BuildError> {
        Ok(LtIvazCancelDeclarationsRequestEntriesItem {
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
