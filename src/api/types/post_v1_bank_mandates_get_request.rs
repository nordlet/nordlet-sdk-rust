pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankMandatesGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1BankMandatesGetRequest {
    pub fn builder() -> PostV1BankMandatesGetRequestBuilder {
        <PostV1BankMandatesGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankMandatesGetRequestBuilder {
    id: Option<String>,
}

impl PostV1BankMandatesGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankMandatesGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankMandatesGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1BankMandatesGetRequest, BuildError> {
        Ok(PostV1BankMandatesGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
