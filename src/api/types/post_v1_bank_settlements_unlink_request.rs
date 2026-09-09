pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankSettlementsUnlinkRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1BankSettlementsUnlinkRequest {
    pub fn builder() -> PostV1BankSettlementsUnlinkRequestBuilder {
        <PostV1BankSettlementsUnlinkRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankSettlementsUnlinkRequestBuilder {
    id: Option<String>,
}

impl PostV1BankSettlementsUnlinkRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankSettlementsUnlinkRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankSettlementsUnlinkRequestBuilder::id)
    pub fn build(self) -> Result<PostV1BankSettlementsUnlinkRequest, BuildError> {
        Ok(PostV1BankSettlementsUnlinkRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
