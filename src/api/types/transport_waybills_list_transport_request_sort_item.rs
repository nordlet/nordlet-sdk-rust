pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WaybillsListTransportRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<WaybillsListTransportRequestSortItemDir>,
}

impl WaybillsListTransportRequestSortItem {
    pub fn builder() -> WaybillsListTransportRequestSortItemBuilder {
        <WaybillsListTransportRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaybillsListTransportRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<WaybillsListTransportRequestSortItemDir>,
}

impl WaybillsListTransportRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: WaybillsListTransportRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WaybillsListTransportRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](WaybillsListTransportRequestSortItemBuilder::field)
    pub fn build(self) -> Result<WaybillsListTransportRequestSortItem, BuildError> {
        Ok(WaybillsListTransportRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
