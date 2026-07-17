pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ConsolidationGroupsCreateRequest {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "presentationCurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_currency: Option<String>,
}

impl PostV1ConsolidationGroupsCreateRequest {
    pub fn builder() -> PostV1ConsolidationGroupsCreateRequestBuilder {
        <PostV1ConsolidationGroupsCreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ConsolidationGroupsCreateRequestBuilder {
    name: Option<String>,
    presentation_currency: Option<String>,
}

impl PostV1ConsolidationGroupsCreateRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn presentation_currency(mut self, value: impl Into<String>) -> Self {
        self.presentation_currency = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ConsolidationGroupsCreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1ConsolidationGroupsCreateRequestBuilder::name)
    pub fn build(self) -> Result<PostV1ConsolidationGroupsCreateRequest, BuildError> {
        Ok(PostV1ConsolidationGroupsCreateRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            presentation_currency: self.presentation_currency,
        })
    }
}
