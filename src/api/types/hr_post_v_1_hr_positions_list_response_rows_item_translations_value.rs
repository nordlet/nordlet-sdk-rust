pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1HrPositionsListResponseRowsItemTranslationsValue {
    #[serde(default)]
    pub name: String,
}

impl PostV1HrPositionsListResponseRowsItemTranslationsValue {
    pub fn builder() -> PostV1HrPositionsListResponseRowsItemTranslationsValueBuilder {
        <PostV1HrPositionsListResponseRowsItemTranslationsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrPositionsListResponseRowsItemTranslationsValueBuilder {
    name: Option<String>,
}

impl PostV1HrPositionsListResponseRowsItemTranslationsValueBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrPositionsListResponseRowsItemTranslationsValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PostV1HrPositionsListResponseRowsItemTranslationsValueBuilder::name)
    pub fn build(
        self,
    ) -> Result<PostV1HrPositionsListResponseRowsItemTranslationsValue, BuildError> {
        Ok(PostV1HrPositionsListResponseRowsItemTranslationsValue {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
