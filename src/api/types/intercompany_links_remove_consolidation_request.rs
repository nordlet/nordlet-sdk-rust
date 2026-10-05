pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntercompanyLinksRemoveConsolidationRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(default)]
    pub id: String,
}

impl IntercompanyLinksRemoveConsolidationRequest {
    pub fn builder() -> IntercompanyLinksRemoveConsolidationRequestBuilder {
        <IntercompanyLinksRemoveConsolidationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntercompanyLinksRemoveConsolidationRequestBuilder {
    group_id: Option<String>,
    id: Option<String>,
}

impl IntercompanyLinksRemoveConsolidationRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IntercompanyLinksRemoveConsolidationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](IntercompanyLinksRemoveConsolidationRequestBuilder::group_id)
    /// - [`id`](IntercompanyLinksRemoveConsolidationRequestBuilder::id)
    pub fn build(self) -> Result<IntercompanyLinksRemoveConsolidationRequest, BuildError> {
        Ok(IntercompanyLinksRemoveConsolidationRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
