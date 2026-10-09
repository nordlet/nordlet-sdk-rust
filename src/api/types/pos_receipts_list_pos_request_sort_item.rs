pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiptsListPosRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<ReceiptsListPosRequestSortItemDir>,
}

impl ReceiptsListPosRequestSortItem {
    pub fn builder() -> ReceiptsListPosRequestSortItemBuilder {
        <ReceiptsListPosRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptsListPosRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<ReceiptsListPosRequestSortItemDir>,
}

impl ReceiptsListPosRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: ReceiptsListPosRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceiptsListPosRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ReceiptsListPosRequestSortItemBuilder::field)
    pub fn build(self) -> Result<ReceiptsListPosRequestSortItem, BuildError> {
        Ok(ReceiptsListPosRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
