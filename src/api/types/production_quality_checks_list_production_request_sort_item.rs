pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QualityChecksListProductionRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<QualityChecksListProductionRequestSortItemDir>,
}

impl QualityChecksListProductionRequestSortItem {
    pub fn builder() -> QualityChecksListProductionRequestSortItemBuilder {
        <QualityChecksListProductionRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QualityChecksListProductionRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<QualityChecksListProductionRequestSortItemDir>,
}

impl QualityChecksListProductionRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: QualityChecksListProductionRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QualityChecksListProductionRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](QualityChecksListProductionRequestSortItemBuilder::field)
    pub fn build(self) -> Result<QualityChecksListProductionRequestSortItem, BuildError> {
        Ok(QualityChecksListProductionRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
