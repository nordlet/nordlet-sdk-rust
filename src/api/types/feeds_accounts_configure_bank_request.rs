pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedsAccountsConfigureBankRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "importTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_template_id: Option<String>,
    #[serde(rename = "syncSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_schedule: Option<FeedsAccountsConfigureBankRequestSyncSchedule>,
}

impl FeedsAccountsConfigureBankRequest {
    pub fn builder() -> FeedsAccountsConfigureBankRequestBuilder {
        <FeedsAccountsConfigureBankRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedsAccountsConfigureBankRequestBuilder {
    id: Option<String>,
    import_template_id: Option<String>,
    sync_schedule: Option<FeedsAccountsConfigureBankRequestSyncSchedule>,
}

impl FeedsAccountsConfigureBankRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn import_template_id(mut self, value: impl Into<String>) -> Self {
        self.import_template_id = Some(value.into());
        self
    }

    pub fn sync_schedule(mut self, value: FeedsAccountsConfigureBankRequestSyncSchedule) -> Self {
        self.sync_schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeedsAccountsConfigureBankRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](FeedsAccountsConfigureBankRequestBuilder::id)
    pub fn build(self) -> Result<FeedsAccountsConfigureBankRequest, BuildError> {
        Ok(FeedsAccountsConfigureBankRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            import_template_id: self.import_template_id,
            sync_schedule: self.sync_schedule,
        })
    }
}
