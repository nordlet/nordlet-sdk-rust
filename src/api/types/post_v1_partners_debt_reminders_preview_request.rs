pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PartnersDebtRemindersPreviewRequest {}

impl PostV1PartnersDebtRemindersPreviewRequest {
    pub fn builder() -> PostV1PartnersDebtRemindersPreviewRequestBuilder {
        <PostV1PartnersDebtRemindersPreviewRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersDebtRemindersPreviewRequestBuilder {}

impl PostV1PartnersDebtRemindersPreviewRequestBuilder {
    /// Consumes the builder and constructs a [`PostV1PartnersDebtRemindersPreviewRequest`].
    pub fn build(self) -> Result<PostV1PartnersDebtRemindersPreviewRequest, BuildError> {
        Ok(PostV1PartnersDebtRemindersPreviewRequest {})
    }
}
