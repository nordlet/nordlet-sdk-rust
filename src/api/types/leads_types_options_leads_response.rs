pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TypesOptionsLeadsResponse {
    #[serde(default)]
    pub rows: Vec<TypesOptionsLeadsResponseRowsItem>,
}

impl TypesOptionsLeadsResponse {
    pub fn builder() -> TypesOptionsLeadsResponseBuilder {
        <TypesOptionsLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesOptionsLeadsResponseBuilder {
    rows: Option<Vec<TypesOptionsLeadsResponseRowsItem>>,
}

impl TypesOptionsLeadsResponseBuilder {
    pub fn rows(mut self, value: Vec<TypesOptionsLeadsResponseRowsItem>) -> Self {
        self.rows = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TypesOptionsLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`rows`](TypesOptionsLeadsResponseBuilder::rows)
    pub fn build(self) -> Result<TypesOptionsLeadsResponse, BuildError> {
        Ok(TypesOptionsLeadsResponse {
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
        })
    }
}
