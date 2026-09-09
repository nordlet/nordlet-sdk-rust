pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1BankMatchRulesDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1BankMatchRulesDeleteResponse {
    pub fn builder() -> PostV1BankMatchRulesDeleteResponseBuilder {
        <PostV1BankMatchRulesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankMatchRulesDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1BankMatchRulesDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankMatchRulesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1BankMatchRulesDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1BankMatchRulesDeleteResponse, BuildError> {
        Ok(PostV1BankMatchRulesDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
