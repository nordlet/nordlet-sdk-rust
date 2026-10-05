pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IncapacityCertificatesListHrRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<IncapacityCertificatesListHrRequestSortItemDir>,
}

impl IncapacityCertificatesListHrRequestSortItem {
    pub fn builder() -> IncapacityCertificatesListHrRequestSortItemBuilder {
        <IncapacityCertificatesListHrRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IncapacityCertificatesListHrRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<IncapacityCertificatesListHrRequestSortItemDir>,
}

impl IncapacityCertificatesListHrRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: IncapacityCertificatesListHrRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IncapacityCertificatesListHrRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](IncapacityCertificatesListHrRequestSortItemBuilder::field)
    pub fn build(self) -> Result<IncapacityCertificatesListHrRequestSortItem, BuildError> {
        Ok(IncapacityCertificatesListHrRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
