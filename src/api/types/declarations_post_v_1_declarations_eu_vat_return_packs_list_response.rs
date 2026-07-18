pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuVatReturnPacksListResponse {
    #[serde(default)]
    pub packs: Vec<PostV1DeclarationsEuVatReturnPacksListResponsePacksItem>,
}

impl PostV1DeclarationsEuVatReturnPacksListResponse {
    pub fn builder() -> PostV1DeclarationsEuVatReturnPacksListResponseBuilder {
        <PostV1DeclarationsEuVatReturnPacksListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuVatReturnPacksListResponseBuilder {
    packs: Option<Vec<PostV1DeclarationsEuVatReturnPacksListResponsePacksItem>>,
}

impl PostV1DeclarationsEuVatReturnPacksListResponseBuilder {
    pub fn packs(
        mut self,
        value: Vec<PostV1DeclarationsEuVatReturnPacksListResponsePacksItem>,
    ) -> Self {
        self.packs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuVatReturnPacksListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`packs`](PostV1DeclarationsEuVatReturnPacksListResponseBuilder::packs)
    pub fn build(self) -> Result<PostV1DeclarationsEuVatReturnPacksListResponse, BuildError> {
        Ok(PostV1DeclarationsEuVatReturnPacksListResponse {
            packs: self
                .packs
                .ok_or_else(|| BuildError::missing_field("packs"))?,
        })
    }
}
