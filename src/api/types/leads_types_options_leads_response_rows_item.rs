pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TypesOptionsLeadsResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl TypesOptionsLeadsResponseRowsItem {
    pub fn builder() -> TypesOptionsLeadsResponseRowsItemBuilder {
        <TypesOptionsLeadsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesOptionsLeadsResponseRowsItemBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl TypesOptionsLeadsResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TypesOptionsLeadsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TypesOptionsLeadsResponseRowsItemBuilder::id)
    /// - [`name`](TypesOptionsLeadsResponseRowsItemBuilder::name)
    pub fn build(self) -> Result<TypesOptionsLeadsResponseRowsItem, BuildError> {
        Ok(TypesOptionsLeadsResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
