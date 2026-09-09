pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankMatchRulesDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1BankMatchRulesDeleteRequest {
    pub fn builder() -> PostV1BankMatchRulesDeleteRequestBuilder {
        <PostV1BankMatchRulesDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankMatchRulesDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1BankMatchRulesDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankMatchRulesDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankMatchRulesDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1BankMatchRulesDeleteRequest, BuildError> {
        Ok(PostV1BankMatchRulesDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
