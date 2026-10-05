pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContractsListHrRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ContractsListHrRequestSortItemDir>,
}

impl ContractsListHrRequestSortItem {
    pub fn builder() -> ContractsListHrRequestSortItemBuilder {
        <ContractsListHrRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContractsListHrRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ContractsListHrRequestSortItemDir>,
}

impl ContractsListHrRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ContractsListHrRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContractsListHrRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ContractsListHrRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ContractsListHrRequestSortItem, BuildError> {
        Ok(ContractsListHrRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
