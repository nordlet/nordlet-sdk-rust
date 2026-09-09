pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogUnitsOptionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<PostV1CatalogUnitsOptionsRequestLocale>,
}

impl PostV1CatalogUnitsOptionsRequest {
    pub fn builder() -> PostV1CatalogUnitsOptionsRequestBuilder {
        <PostV1CatalogUnitsOptionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogUnitsOptionsRequestBuilder {
    locale: Option<PostV1CatalogUnitsOptionsRequestLocale>,
}

impl PostV1CatalogUnitsOptionsRequestBuilder {
    pub fn locale(mut self, value: PostV1CatalogUnitsOptionsRequestLocale) -> Self {
        self.locale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogUnitsOptionsRequest`].
    pub fn build(self) -> Result<PostV1CatalogUnitsOptionsRequest, BuildError> {
        Ok(PostV1CatalogUnitsOptionsRequest {
            locale: self.locale,
        })
    }
}
