pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1CatalogUnitsDeleteResponse {
    #[serde(default)]
    pub id: String,
}

impl PostV1CatalogUnitsDeleteResponse {
    pub fn builder() -> PostV1CatalogUnitsDeleteResponseBuilder {
        <PostV1CatalogUnitsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1CatalogUnitsDeleteResponseBuilder {
    id: Option<String>,
}

impl PostV1CatalogUnitsDeleteResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1CatalogUnitsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1CatalogUnitsDeleteResponseBuilder::id)
    pub fn build(self) -> Result<PostV1CatalogUnitsDeleteResponse, BuildError> {
        Ok(PostV1CatalogUnitsDeleteResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
