pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankImportTemplatesGetRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1BankImportTemplatesGetRequest {
    pub fn builder() -> PostV1BankImportTemplatesGetRequestBuilder {
        <PostV1BankImportTemplatesGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankImportTemplatesGetRequestBuilder {
    id: Option<String>,
}

impl PostV1BankImportTemplatesGetRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankImportTemplatesGetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankImportTemplatesGetRequestBuilder::id)
    pub fn build(self) -> Result<PostV1BankImportTemplatesGetRequest, BuildError> {
        Ok(PostV1BankImportTemplatesGetRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
