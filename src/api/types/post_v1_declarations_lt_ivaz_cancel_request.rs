pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtIvazCancelRequest {
    #[serde(default)]
    pub entries: Vec<PostV1DeclarationsLtIvazCancelRequestEntriesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persist: Option<bool>,
}

impl PostV1DeclarationsLtIvazCancelRequest {
    pub fn builder() -> PostV1DeclarationsLtIvazCancelRequestBuilder {
        <PostV1DeclarationsLtIvazCancelRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtIvazCancelRequestBuilder {
    entries: Option<Vec<PostV1DeclarationsLtIvazCancelRequestEntriesItem>>,
    persist: Option<bool>,
}

impl PostV1DeclarationsLtIvazCancelRequestBuilder {
    pub fn entries(mut self, value: Vec<PostV1DeclarationsLtIvazCancelRequestEntriesItem>) -> Self {
        self.entries = Some(value);
        self
    }

    pub fn persist(mut self, value: bool) -> Self {
        self.persist = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtIvazCancelRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entries`](PostV1DeclarationsLtIvazCancelRequestBuilder::entries)
    pub fn build(self) -> Result<PostV1DeclarationsLtIvazCancelRequest, BuildError> {
        Ok(PostV1DeclarationsLtIvazCancelRequest {
            entries: self
                .entries
                .ok_or_else(|| BuildError::missing_field("entries"))?,
            persist: self.persist,
        })
    }
}
