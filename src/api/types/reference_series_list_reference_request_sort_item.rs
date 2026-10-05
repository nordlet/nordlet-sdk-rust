pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SeriesListReferenceRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<SeriesListReferenceRequestSortItemDir>,
}

impl SeriesListReferenceRequestSortItem {
    pub fn builder() -> SeriesListReferenceRequestSortItemBuilder {
        <SeriesListReferenceRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SeriesListReferenceRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<SeriesListReferenceRequestSortItemDir>,
}

impl SeriesListReferenceRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: SeriesListReferenceRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SeriesListReferenceRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](SeriesListReferenceRequestSortItemBuilder::field)
    pub fn build(self) -> Result<SeriesListReferenceRequestSortItem, BuildError> {
        Ok(SeriesListReferenceRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
