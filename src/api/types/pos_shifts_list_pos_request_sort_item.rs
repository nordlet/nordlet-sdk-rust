pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShiftsListPosRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ShiftsListPosRequestSortItemDir>,
}

impl ShiftsListPosRequestSortItem {
    pub fn builder() -> ShiftsListPosRequestSortItemBuilder {
        <ShiftsListPosRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShiftsListPosRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ShiftsListPosRequestSortItemDir>,
}

impl ShiftsListPosRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ShiftsListPosRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ShiftsListPosRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ShiftsListPosRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ShiftsListPosRequestSortItem, BuildError> {
        Ok(ShiftsListPosRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
