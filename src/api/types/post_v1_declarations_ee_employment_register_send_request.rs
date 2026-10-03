pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEeEmploymentRegisterSendRequest {
    #[serde(rename = "contractId")]
    #[serde(default)]
    pub contract_id: String,
    pub event: PostV1DeclarationsEeEmploymentRegisterSendRequestEvent,
}

impl PostV1DeclarationsEeEmploymentRegisterSendRequest {
    pub fn builder() -> PostV1DeclarationsEeEmploymentRegisterSendRequestBuilder {
        <PostV1DeclarationsEeEmploymentRegisterSendRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEeEmploymentRegisterSendRequestBuilder {
    contract_id: Option<String>,
    event: Option<PostV1DeclarationsEeEmploymentRegisterSendRequestEvent>,
}

impl PostV1DeclarationsEeEmploymentRegisterSendRequestBuilder {
    pub fn contract_id(mut self, value: impl Into<String>) -> Self {
        self.contract_id = Some(value.into());
        self
    }

    pub fn event(mut self, value: PostV1DeclarationsEeEmploymentRegisterSendRequestEvent) -> Self {
        self.event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEeEmploymentRegisterSendRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contract_id`](PostV1DeclarationsEeEmploymentRegisterSendRequestBuilder::contract_id)
    /// - [`event`](PostV1DeclarationsEeEmploymentRegisterSendRequestBuilder::event)
    pub fn build(self) -> Result<PostV1DeclarationsEeEmploymentRegisterSendRequest, BuildError> {
        Ok(PostV1DeclarationsEeEmploymentRegisterSendRequest {
            contract_id: self
                .contract_id
                .ok_or_else(|| BuildError::missing_field("contract_id"))?,
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
        })
    }
}
