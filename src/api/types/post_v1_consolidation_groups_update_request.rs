pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationGroupsUpdateRequest {
    #[serde(rename = "groupId")]
    #[serde(default)]
    pub group_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "presentationCurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_currency: Option<String>,
}

impl PostV1ConsolidationGroupsUpdateRequest {
    pub fn builder() -> PostV1ConsolidationGroupsUpdateRequestBuilder {
        <PostV1ConsolidationGroupsUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationGroupsUpdateRequestBuilder {
    group_id: Option<String>,
    name: Option<String>,
    presentation_currency: Option<String>,
}

impl PostV1ConsolidationGroupsUpdateRequestBuilder {
    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn presentation_currency(mut self, value: impl Into<String>) -> Self {
        self.presentation_currency = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationGroupsUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`group_id`](PostV1ConsolidationGroupsUpdateRequestBuilder::group_id)
    pub fn build(self) -> Result<PostV1ConsolidationGroupsUpdateRequest, BuildError> {
        Ok(PostV1ConsolidationGroupsUpdateRequest {
            group_id: self
                .group_id
                .ok_or_else(|| BuildError::missing_field("group_id"))?,
            name: self.name,
            presentation_currency: self.presentation_currency,
        })
    }
}
