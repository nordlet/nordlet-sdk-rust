pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1AgreementsTypesListResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl PostV1AgreementsTypesListResponseRowsItem {
    pub fn builder() -> PostV1AgreementsTypesListResponseRowsItemBuilder {
        <PostV1AgreementsTypesListResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsTypesListResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
}

impl PostV1AgreementsTypesListResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1AgreementsTypesListResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1AgreementsTypesListResponseRowsItemBuilder::id)
    /// - [`code`](PostV1AgreementsTypesListResponseRowsItemBuilder::code)
    /// - [`name`](PostV1AgreementsTypesListResponseRowsItemBuilder::name)
    pub fn build(self) -> Result<PostV1AgreementsTypesListResponseRowsItem, BuildError> {
        Ok(PostV1AgreementsTypesListResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
