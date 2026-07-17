pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsAgreementsListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1AgreementsAgreementsListRequestSortItemDir>,
}

impl PostV1AgreementsAgreementsListRequestSortItem {
    pub fn builder() -> PostV1AgreementsAgreementsListRequestSortItemBuilder {
        <PostV1AgreementsAgreementsListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsAgreementsListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1AgreementsAgreementsListRequestSortItemDir>,
}

impl PostV1AgreementsAgreementsListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1AgreementsAgreementsListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsAgreementsListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1AgreementsAgreementsListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1AgreementsAgreementsListRequestSortItem, BuildError> {
        Ok(PostV1AgreementsAgreementsListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
