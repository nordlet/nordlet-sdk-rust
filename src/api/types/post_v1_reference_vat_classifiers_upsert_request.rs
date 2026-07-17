pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceVatClassifiersUpsertRequest {
    #[serde(default)]
    pub rows: Vec<PostV1ReferenceVatClassifiersUpsertRequestRowsItem>,
}

impl PostV1ReferenceVatClassifiersUpsertRequest {
    pub fn builder() -> PostV1ReferenceVatClassifiersUpsertRequestBuilder {
        <PostV1ReferenceVatClassifiersUpsertRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceVatClassifiersUpsertRequestBuilder {
    rows: Option<Vec<PostV1ReferenceVatClassifiersUpsertRequestRowsItem>>,
}

impl PostV1ReferenceVatClassifiersUpsertRequestBuilder {
    pub fn rows(mut self, value: Vec<PostV1ReferenceVatClassifiersUpsertRequestRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceVatClassifiersUpsertRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReferenceVatClassifiersUpsertRequestBuilder::rows)
    pub fn build(self) -> Result<PostV1ReferenceVatClassifiersUpsertRequest, BuildError> {
        Ok(PostV1ReferenceVatClassifiersUpsertRequest {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
