pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceVatClassifiersUpsertResponse {
    #[serde(default)]
    pub upserted: i64,
}

impl PostV1ReferenceVatClassifiersUpsertResponse {
    pub fn builder() -> PostV1ReferenceVatClassifiersUpsertResponseBuilder {
        <PostV1ReferenceVatClassifiersUpsertResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceVatClassifiersUpsertResponseBuilder {
    upserted: Option<i64>,
}

impl PostV1ReferenceVatClassifiersUpsertResponseBuilder {
    pub fn upserted(mut self, value: i64) -> Self {
        self.upserted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceVatClassifiersUpsertResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`upserted`](PostV1ReferenceVatClassifiersUpsertResponseBuilder::upserted)
    pub fn build(self) -> Result<PostV1ReferenceVatClassifiersUpsertResponse, BuildError> {
        Ok(PostV1ReferenceVatClassifiersUpsertResponse {
            upserted: self
                .upserted
                .ok_or_else(|| BuildError::missing_field("upserted"))?,
        })
    }
}
