pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogUnitsDeleteRequest {
    #[serde(default)]
    pub id: String,
}

impl PostV1CatalogUnitsDeleteRequest {
    pub fn builder() -> PostV1CatalogUnitsDeleteRequestBuilder {
        <PostV1CatalogUnitsDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogUnitsDeleteRequestBuilder {
    id: Option<String>,
}

impl PostV1CatalogUnitsDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogUnitsDeleteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogUnitsDeleteRequestBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogUnitsDeleteRequest, BuildError> {
        Ok(PostV1CatalogUnitsDeleteRequest {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
