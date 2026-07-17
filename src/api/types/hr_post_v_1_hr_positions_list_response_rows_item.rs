pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostV1HrPositionsListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translations:
        Option<HashMap<String, Option<PostV1HrPositionsListResponseRowsItemTranslationsValue>>>,
}

impl PostV1HrPositionsListResponseRowsItem {
    pub fn builder() -> PostV1HrPositionsListResponseRowsItemBuilder {
        <PostV1HrPositionsListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrPositionsListResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    translations:
        Option<HashMap<String, Option<PostV1HrPositionsListResponseRowsItemTranslationsValue>>>,
}

impl PostV1HrPositionsListResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn translations(
        mut self,
        value: HashMap<String, Option<PostV1HrPositionsListResponseRowsItemTranslationsValue>>,
    ) -> Self {
        self.translations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrPositionsListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1HrPositionsListResponseRowsItemBuilder::id)
    /// - [`name`](PostV1HrPositionsListResponseRowsItemBuilder::name)
    pub fn build(self) -> Result<PostV1HrPositionsListResponseRowsItem, BuildError> {
        Ok(PostV1HrPositionsListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            translations: self.translations,
        })
    }
}
