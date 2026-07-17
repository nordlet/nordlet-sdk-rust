pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionOrdersCompleteRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "componentsAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components_account_code: Option<String>,
    #[serde(rename = "finishedAccountCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_account_code: Option<String>,
}

impl PostV1ProductionOrdersCompleteRequest {
    pub fn builder() -> PostV1ProductionOrdersCompleteRequestBuilder {
        <PostV1ProductionOrdersCompleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionOrdersCompleteRequestBuilder {
    id: Option<String>,
    components_account_code: Option<String>,
    finished_account_code: Option<String>,
}

impl PostV1ProductionOrdersCompleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn components_account_code(mut self, value: impl Into<String>) -> Self {
        self.components_account_code = Some(value.into());
        self
    }

    pub fn finished_account_code(mut self, value: impl Into<String>) -> Self {
        self.finished_account_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionOrdersCompleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionOrdersCompleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1ProductionOrdersCompleteRequest, BuildError> {
        Ok(PostV1ProductionOrdersCompleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            components_account_code: self.components_account_code,
            finished_account_code: self.finished_account_code,
        })
    }
}
