pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnitsListReferenceRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<UnitsListReferenceRequestSortItemDir>,
}

impl UnitsListReferenceRequestSortItem {
    pub fn builder() -> UnitsListReferenceRequestSortItemBuilder {
        <UnitsListReferenceRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnitsListReferenceRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<UnitsListReferenceRequestSortItemDir>,
}

impl UnitsListReferenceRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: UnitsListReferenceRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnitsListReferenceRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](UnitsListReferenceRequestSortItemBuilder::field)
    pub fn build(self) -> Result<UnitsListReferenceRequestSortItem, BuildError> {
        Ok(UnitsListReferenceRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
