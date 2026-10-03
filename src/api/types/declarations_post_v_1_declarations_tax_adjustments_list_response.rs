pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsTaxAdjustmentsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsTaxAdjustmentsListResponseRowsItem>,
}

impl PostV1DeclarationsTaxAdjustmentsListResponse {
    pub fn builder() -> PostV1DeclarationsTaxAdjustmentsListResponseBuilder {
        <PostV1DeclarationsTaxAdjustmentsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsTaxAdjustmentsListResponseBuilder {
    rows: Option<Vec<PostV1DeclarationsTaxAdjustmentsListResponseRowsItem>>,
}

impl PostV1DeclarationsTaxAdjustmentsListResponseBuilder {
    pub fn rows(
        mut self,
        value: Vec<PostV1DeclarationsTaxAdjustmentsListResponseRowsItem>,
    ) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsTaxAdjustmentsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1DeclarationsTaxAdjustmentsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1DeclarationsTaxAdjustmentsListResponse, BuildError> {
        Ok(PostV1DeclarationsTaxAdjustmentsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
