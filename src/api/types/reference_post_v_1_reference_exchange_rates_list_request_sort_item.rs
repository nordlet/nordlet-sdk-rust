pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceExchangeRatesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1ReferenceExchangeRatesListRequestSortItemDir>,
}

impl PostV1ReferenceExchangeRatesListRequestSortItem {
    pub fn builder() -> PostV1ReferenceExchangeRatesListRequestSortItemBuilder {
        <PostV1ReferenceExchangeRatesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceExchangeRatesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1ReferenceExchangeRatesListRequestSortItemDir>,
}

impl PostV1ReferenceExchangeRatesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PostV1ReferenceExchangeRatesListRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceExchangeRatesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceExchangeRatesListRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PostV1ReferenceExchangeRatesListRequestSortItem, BuildError> {
        Ok(PostV1ReferenceExchangeRatesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
