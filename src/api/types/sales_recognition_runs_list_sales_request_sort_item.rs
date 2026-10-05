pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecognitionRunsListSalesRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<RecognitionRunsListSalesRequestSortItemDir>,
}

impl RecognitionRunsListSalesRequestSortItem {
    pub fn builder() -> RecognitionRunsListSalesRequestSortItemBuilder {
        <RecognitionRunsListSalesRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionRunsListSalesRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<RecognitionRunsListSalesRequestSortItemDir>,
}

impl RecognitionRunsListSalesRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: RecognitionRunsListSalesRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecognitionRunsListSalesRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RecognitionRunsListSalesRequestSortItemBuilder::field)
    pub fn build(self) -> Result<RecognitionRunsListSalesRequestSortItem, BuildError> {
        Ok(RecognitionRunsListSalesRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
