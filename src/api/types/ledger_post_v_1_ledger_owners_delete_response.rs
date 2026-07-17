pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LedgerOwnersDeleteResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1LedgerOwnersDeleteResponse {
    pub fn builder() -> PostV1LedgerOwnersDeleteResponseBuilder {
        <PostV1LedgerOwnersDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LedgerOwnersDeleteResponseBuilder {
    id: Option<String>,
    deleted: Option<bool>,
}

impl PostV1LedgerOwnersDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1LedgerOwnersDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LedgerOwnersDeleteResponseBuilder::id)
    /// - [`deleted`](PostV1LedgerOwnersDeleteResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1LedgerOwnersDeleteResponse, BuildError> {
        Ok(PostV1LedgerOwnersDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
