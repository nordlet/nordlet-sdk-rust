pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TypesListAgreementsRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<TypesListAgreementsRequestSortItemDir>,
}

impl TypesListAgreementsRequestSortItem {
    pub fn builder() -> TypesListAgreementsRequestSortItemBuilder {
        <TypesListAgreementsRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesListAgreementsRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<TypesListAgreementsRequestSortItemDir>,
}

impl TypesListAgreementsRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: TypesListAgreementsRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TypesListAgreementsRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](TypesListAgreementsRequestSortItemBuilder::field)
    pub fn build(self) -> Result<TypesListAgreementsRequestSortItem, BuildError> {
        Ok(TypesListAgreementsRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
