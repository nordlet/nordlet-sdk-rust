pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EeEmploymentRegisterSendDeclarationsRequest {
    #[serde(rename = "contractId")]
    #[serde(default)]
    pub contract_id: String,
    pub event: EeEmploymentRegisterSendDeclarationsRequestEvent,
}

impl EeEmploymentRegisterSendDeclarationsRequest {
    pub fn builder() -> EeEmploymentRegisterSendDeclarationsRequestBuilder {
        <EeEmploymentRegisterSendDeclarationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EeEmploymentRegisterSendDeclarationsRequestBuilder {
    contract_id: Option<String>,
    event: Option<EeEmploymentRegisterSendDeclarationsRequestEvent>,
}

impl EeEmploymentRegisterSendDeclarationsRequestBuilder {
    pub fn contract_id(mut self, value: impl Into<String>) -> Self {
        self.contract_id = Some(value.into());
        self
    }

    pub fn event(mut self, value: EeEmploymentRegisterSendDeclarationsRequestEvent) -> Self {
        self.event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EeEmploymentRegisterSendDeclarationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`contract_id`](EeEmploymentRegisterSendDeclarationsRequestBuilder::contract_id)
    /// - [`event`](EeEmploymentRegisterSendDeclarationsRequestBuilder::event)
    pub fn build(self) -> Result<EeEmploymentRegisterSendDeclarationsRequest, BuildError> {
        Ok(EeEmploymentRegisterSendDeclarationsRequest {
            contract_id: self
                .contract_id
                .ok_or_else(|| BuildError::missing_field("contract_id"))?,
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
        })
    }
}
