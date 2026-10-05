pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BomsListProductionRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<BomsListProductionRequestSortItemDir>,
}

impl BomsListProductionRequestSortItem {
    pub fn builder() -> BomsListProductionRequestSortItemBuilder {
        <BomsListProductionRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BomsListProductionRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<BomsListProductionRequestSortItemDir>,
}

impl BomsListProductionRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: BomsListProductionRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BomsListProductionRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](BomsListProductionRequestSortItemBuilder::field)
    pub fn build(self) -> Result<BomsListProductionRequestSortItem, BuildError> {
        Ok(BomsListProductionRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
