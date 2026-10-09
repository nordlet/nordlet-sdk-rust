pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PerDiemRatesListHrRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<PerDiemRatesListHrRequestSortItemDir>,
}

impl PerDiemRatesListHrRequestSortItem {
    pub fn builder() -> PerDiemRatesListHrRequestSortItemBuilder {
        <PerDiemRatesListHrRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PerDiemRatesListHrRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<PerDiemRatesListHrRequestSortItemDir>,
}

impl PerDiemRatesListHrRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: PerDiemRatesListHrRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PerDiemRatesListHrRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PerDiemRatesListHrRequestSortItemBuilder::field)
    pub fn build(self) -> Result<PerDiemRatesListHrRequestSortItem, BuildError> {
        Ok(PerDiemRatesListHrRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
