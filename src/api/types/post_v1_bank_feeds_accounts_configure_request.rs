pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankFeedsAccountsConfigureRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "importTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_template_id: Option<String>,
    #[serde(rename = "syncSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_schedule: Option<PostV1BankFeedsAccountsConfigureRequestSyncSchedule>,
}

impl PostV1BankFeedsAccountsConfigureRequest {
    pub fn builder() -> PostV1BankFeedsAccountsConfigureRequestBuilder {
        <PostV1BankFeedsAccountsConfigureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankFeedsAccountsConfigureRequestBuilder {
    id: Option<String>,
    import_template_id: Option<String>,
    sync_schedule: Option<PostV1BankFeedsAccountsConfigureRequestSyncSchedule>,
}

impl PostV1BankFeedsAccountsConfigureRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn import_template_id(mut self, value: impl Into<String>) -> Self {
        self.import_template_id = Some(value.into());
        self
    }

    pub fn sync_schedule(
        mut self,
        value: PostV1BankFeedsAccountsConfigureRequestSyncSchedule,
    ) -> Self {
        self.sync_schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankFeedsAccountsConfigureRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankFeedsAccountsConfigureRequestBuilder::id)
    pub fn build(self) -> Result<PostV1BankFeedsAccountsConfigureRequest, BuildError> {
        Ok(PostV1BankFeedsAccountsConfigureRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            import_template_id: self.import_template_id,
            sync_schedule: self.sync_schedule,
        })
    }
}
