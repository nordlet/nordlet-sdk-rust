pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrIncapacityCertificatesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1HrIncapacityCertificatesListRequestSortItemDir>,
}

impl PostV1HrIncapacityCertificatesListRequestSortItem {
    pub fn builder() -> PostV1HrIncapacityCertificatesListRequestSortItemBuilder {
        <PostV1HrIncapacityCertificatesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrIncapacityCertificatesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1HrIncapacityCertificatesListRequestSortItemDir>,
}

impl PostV1HrIncapacityCertificatesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1HrIncapacityCertificatesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrIncapacityCertificatesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1HrIncapacityCertificatesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1HrIncapacityCertificatesListRequestSortItem, BuildError> {
        Ok(PostV1HrIncapacityCertificatesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
