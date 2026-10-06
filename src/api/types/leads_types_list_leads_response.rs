pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TypesListLeadsResponse {
    #[serde(default)]
    pub rows: Vec<TypesListLeadsResponseRowsItem>,
}

impl TypesListLeadsResponse {
    pub fn builder() -> TypesListLeadsResponseBuilder {
        <TypesListLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesListLeadsResponseBuilder {
    rows: Option<Vec<TypesListLeadsResponseRowsItem>>,
}

impl TypesListLeadsResponseBuilder {
    pub fn rows(mut self, value: Vec<TypesListLeadsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TypesListLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](TypesListLeadsResponseBuilder::rows)
    pub fn build(self) -> Result<TypesListLeadsResponse, BuildError> {
        Ok(TypesListLeadsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
