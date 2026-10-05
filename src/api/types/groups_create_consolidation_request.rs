pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GroupsCreateConsolidationRequest {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "presentationCurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_currency: Option<String>,
}

impl GroupsCreateConsolidationRequest {
    pub fn builder() -> GroupsCreateConsolidationRequestBuilder {
        <GroupsCreateConsolidationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GroupsCreateConsolidationRequestBuilder {
    name: Option<String>,
    presentation_currency: Option<String>,
}

impl GroupsCreateConsolidationRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn presentation_currency(mut self, value: impl Into<String>) -> Self {
        self.presentation_currency = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GroupsCreateConsolidationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](GroupsCreateConsolidationRequestBuilder::name)
    pub fn build(self) -> Result<GroupsCreateConsolidationRequest, BuildError> {
        Ok(GroupsCreateConsolidationRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            presentation_currency: self.presentation_currency,
        })
    }
}
