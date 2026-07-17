pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReferenceExchangeRatesOverridesListRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PostV1ReferenceExchangeRatesOverridesListRequestSortItemDir>,
}

impl PostV1ReferenceExchangeRatesOverridesListRequestSortItem {
    pub fn builder() -> PostV1ReferenceExchangeRatesOverridesListRequestSortItemBuilder {
        <PostV1ReferenceExchangeRatesOverridesListRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReferenceExchangeRatesOverridesListRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PostV1ReferenceExchangeRatesOverridesListRequestSortItemDir>,
}

impl PostV1ReferenceExchangeRatesOverridesListRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(
        mut self,
        value: PostV1ReferenceExchangeRatesOverridesListRequestSortItemDir,
    ) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1ReferenceExchangeRatesOverridesListRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1ReferenceExchangeRatesOverridesListRequestSortItemBuilder::field)
    pub fn build(
        self,
    ) -> Result<PostV1ReferenceExchangeRatesOverridesListRequestSortItem, BuildError> {
        Ok(PostV1ReferenceExchangeRatesOverridesListRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
