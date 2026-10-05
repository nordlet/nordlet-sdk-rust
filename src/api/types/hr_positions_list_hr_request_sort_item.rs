pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PositionsListHrRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PositionsListHrRequestSortItemDir>,
}

impl PositionsListHrRequestSortItem {
    pub fn builder() -> PositionsListHrRequestSortItemBuilder {
        <PositionsListHrRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PositionsListHrRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PositionsListHrRequestSortItemDir>,
}

impl PositionsListHrRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PositionsListHrRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PositionsListHrRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PositionsListHrRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PositionsListHrRequestSortItem, BuildError> {
        Ok(PositionsListHrRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
