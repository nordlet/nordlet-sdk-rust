pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceCurrenciesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1ReferenceCurrenciesListRequestSortItemDir>,
}

impl PostV1ReferenceCurrenciesListRequestSortItem {
    pub fn builder() -> PostV1ReferenceCurrenciesListRequestSortItemBuilder {
        <PostV1ReferenceCurrenciesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceCurrenciesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1ReferenceCurrenciesListRequestSortItemDir>,
}

impl PostV1ReferenceCurrenciesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1ReferenceCurrenciesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceCurrenciesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceCurrenciesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1ReferenceCurrenciesListRequestSortItem, BuildError> {
        Ok(PostV1ReferenceCurrenciesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
