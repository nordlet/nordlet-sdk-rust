pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceCnCodesUpsertRequest {
    #[serde(default)]
    pub rows: Vec<PostV1ReferenceCnCodesUpsertRequestRowsItem>,
}

impl PostV1ReferenceCnCodesUpsertRequest {
    pub fn builder() -> PostV1ReferenceCnCodesUpsertRequestBuilder {
        <PostV1ReferenceCnCodesUpsertRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceCnCodesUpsertRequestBuilder {
    rows: Option<Vec<PostV1ReferenceCnCodesUpsertRequestRowsItem>>,
}

impl PostV1ReferenceCnCodesUpsertRequestBuilder {
    pub fn rows(mut self, value: Vec<PostV1ReferenceCnCodesUpsertRequestRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceCnCodesUpsertRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1ReferenceCnCodesUpsertRequestBuilder::rows)
    pub fn build(self) -> Result<PostV1ReferenceCnCodesUpsertRequest, BuildError> {
        Ok(PostV1ReferenceCnCodesUpsertRequest {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
