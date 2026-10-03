pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsTaxPaymentsListResponse {
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsTaxPaymentsListResponseRowsItem>,
}

impl PostV1DeclarationsTaxPaymentsListResponse {
    pub fn builder() -> PostV1DeclarationsTaxPaymentsListResponseBuilder {
        <PostV1DeclarationsTaxPaymentsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsTaxPaymentsListResponseBuilder {
    rows: Option<Vec<PostV1DeclarationsTaxPaymentsListResponseRowsItem>>,
}

impl PostV1DeclarationsTaxPaymentsListResponseBuilder {
    pub fn rows(mut self, value: Vec<PostV1DeclarationsTaxPaymentsListResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsTaxPaymentsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](PostV1DeclarationsTaxPaymentsListResponseBuilder::rows)
    pub fn build(self) -> Result<PostV1DeclarationsTaxPaymentsListResponse, BuildError> {
        Ok(PostV1DeclarationsTaxPaymentsListResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
