pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IntercompanyLinksListConsolidationRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
}

impl IntercompanyLinksListConsolidationRequest {
    pub fn builder() -> IntercompanyLinksListConsolidationRequestBuilder {
        <IntercompanyLinksListConsolidationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IntercompanyLinksListConsolidationRequestBuilder {
    group_id: Option<String>,
}

impl IntercompanyLinksListConsolidationRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IntercompanyLinksListConsolidationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](IntercompanyLinksListConsolidationRequestBuilder::group_id)
    pub fn build(self) -> Result<IntercompanyLinksListConsolidationRequest, BuildError> {
        Ok(IntercompanyLinksListConsolidationRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
        })
    }
}
