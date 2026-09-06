pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankImportTemplatesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1BankImportTemplatesDeleteRequest {
    pub fn builder() -> PostV1BankImportTemplatesDeleteRequestBuilder {
        <PostV1BankImportTemplatesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankImportTemplatesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1BankImportTemplatesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankImportTemplatesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankImportTemplatesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1BankImportTemplatesDeleteRequest, BuildError> {
        Ok(PostV1BankImportTemplatesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
