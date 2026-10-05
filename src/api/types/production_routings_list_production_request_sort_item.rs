pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RoutingsListProductionRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<RoutingsListProductionRequestSortItemDir>,
}

impl RoutingsListProductionRequestSortItem {
    pub fn builder() -> RoutingsListProductionRequestSortItemBuilder {
        <RoutingsListProductionRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RoutingsListProductionRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<RoutingsListProductionRequestSortItemDir>,
}

impl RoutingsListProductionRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: RoutingsListProductionRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RoutingsListProductionRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RoutingsListProductionRequestSortItemBuilder::field)
    pub fn build(self) -> Result<RoutingsListProductionRequestSortItem, BuildError> {
        Ok(RoutingsListProductionRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
