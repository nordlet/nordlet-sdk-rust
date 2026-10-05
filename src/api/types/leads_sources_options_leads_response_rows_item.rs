pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourcesOptionsLeadsResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl SourcesOptionsLeadsResponseRowsItem {
    pub fn builder() -> SourcesOptionsLeadsResponseRowsItemBuilder {
        <SourcesOptionsLeadsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourcesOptionsLeadsResponseRowsItemBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl SourcesOptionsLeadsResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SourcesOptionsLeadsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SourcesOptionsLeadsResponseRowsItemBuilder::id)
    /// - [`name`](SourcesOptionsLeadsResponseRowsItemBuilder::name)
    pub fn build(self) -> Result<SourcesOptionsLeadsResponseRowsItem, BuildError> {
        Ok(SourcesOptionsLeadsResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
