pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DevicesListPosRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<DevicesListPosRequestSortItemDir>,
}

impl DevicesListPosRequestSortItem {
    pub fn builder() -> DevicesListPosRequestSortItemBuilder {
        <DevicesListPosRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DevicesListPosRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<DevicesListPosRequestSortItemDir>,
}

impl DevicesListPosRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: DevicesListPosRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DevicesListPosRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](DevicesListPosRequestSortItemBuilder::field)
    pub fn build(self) -> Result<DevicesListPosRequestSortItem, BuildError> {
        Ok(DevicesListPosRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
