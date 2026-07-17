pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankSettlementsGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1BankSettlementsGetRequest {
    pub fn builder() -> PostV1BankSettlementsGetRequestBuilder {
        <PostV1BankSettlementsGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankSettlementsGetRequestBuilder {
    id: Option<String>,
}

impl PostV1BankSettlementsGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankSettlementsGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankSettlementsGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1BankSettlementsGetRequest, BuildError> {
        Ok(PostV1BankSettlementsGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
