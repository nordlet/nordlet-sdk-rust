pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1LeadsSourcesOptionsResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl PostV1LeadsSourcesOptionsResponseRowsItem {
    pub fn builder() -> PostV1LeadsSourcesOptionsResponseRowsItemBuilder {
        <PostV1LeadsSourcesOptionsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1LeadsSourcesOptionsResponseRowsItemBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl PostV1LeadsSourcesOptionsResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1LeadsSourcesOptionsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1LeadsSourcesOptionsResponseRowsItemBuilder::id)
    /// - [`name`](PostV1LeadsSourcesOptionsResponseRowsItemBuilder::name)
    pub fn build(self) -> Result<PostV1LeadsSourcesOptionsResponseRowsItem, BuildError> {
        Ok(PostV1LeadsSourcesOptionsResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
