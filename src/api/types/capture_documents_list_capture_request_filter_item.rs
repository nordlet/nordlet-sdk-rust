pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentsListCaptureRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: DocumentsListCaptureRequestFilterItemOp,
    pub value: DocumentsListCaptureRequestFilterItemValue,
}

impl DocumentsListCaptureRequestFilterItem {
    pub fn builder() -> DocumentsListCaptureRequestFilterItemBuilder {
        <DocumentsListCaptureRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentsListCaptureRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<DocumentsListCaptureRequestFilterItemOp>,
    value: Option<DocumentsListCaptureRequestFilterItemValue>,
}

impl DocumentsListCaptureRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: DocumentsListCaptureRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: DocumentsListCaptureRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentsListCaptureRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](DocumentsListCaptureRequestFilterItemBuilder::field)
    /// - [`op`](DocumentsListCaptureRequestFilterItemBuilder::op)
    /// - [`value`](DocumentsListCaptureRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<DocumentsListCaptureRequestFilterItem, BuildError> {
        Ok(DocumentsListCaptureRequestFilterItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            op: self.op.ok_or_else(|| BuildError::missing_field("op"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
