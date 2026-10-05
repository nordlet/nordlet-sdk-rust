pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecognitionSchedulesListSalesRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<RecognitionSchedulesListSalesRequestSortItemDir>,
}

impl RecognitionSchedulesListSalesRequestSortItem {
    pub fn builder() -> RecognitionSchedulesListSalesRequestSortItemBuilder {
        <RecognitionSchedulesListSalesRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionSchedulesListSalesRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<RecognitionSchedulesListSalesRequestSortItemDir>,
}

impl RecognitionSchedulesListSalesRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: RecognitionSchedulesListSalesRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecognitionSchedulesListSalesRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RecognitionSchedulesListSalesRequestSortItemBuilder::field)
    pub fn build(self) -> Result<RecognitionSchedulesListSalesRequestSortItem, BuildError> {
        Ok(RecognitionSchedulesListSalesRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
