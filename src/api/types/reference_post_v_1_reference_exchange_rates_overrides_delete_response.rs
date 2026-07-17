pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceExchangeRatesOverridesDeleteResponse {
    #[serde(default)]
    pub deleted: bool,
}

impl PostV1ReferenceExchangeRatesOverridesDeleteResponse {
    pub fn builder() -> PostV1ReferenceExchangeRatesOverridesDeleteResponseBuilder {
        <PostV1ReferenceExchangeRatesOverridesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceExchangeRatesOverridesDeleteResponseBuilder {
    deleted: Option<bool>,
}

impl PostV1ReferenceExchangeRatesOverridesDeleteResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceExchangeRatesOverridesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](PostV1ReferenceExchangeRatesOverridesDeleteResponseBuilder::deleted)
    pub fn build(self) -> Result<PostV1ReferenceExchangeRatesOverridesDeleteResponse, BuildError> {
        Ok(PostV1ReferenceExchangeRatesOverridesDeleteResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
