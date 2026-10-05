pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListDocumentSeriesRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ListDocumentSeriesRequestFilterItemOp,
    pub value: ListDocumentSeriesRequestFilterItemValue,
}

impl ListDocumentSeriesRequestFilterItem {
    pub fn builder() -> ListDocumentSeriesRequestFilterItemBuilder {
        <ListDocumentSeriesRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDocumentSeriesRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ListDocumentSeriesRequestFilterItemOp>,
    value: Option<ListDocumentSeriesRequestFilterItemValue>,
}

impl ListDocumentSeriesRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ListDocumentSeriesRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ListDocumentSeriesRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDocumentSeriesRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ListDocumentSeriesRequestFilterItemBuilder::field)
    /// - [`op`](ListDocumentSeriesRequestFilterItemBuilder::op)
    /// - [`value`](ListDocumentSeriesRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ListDocumentSeriesRequestFilterItem, BuildError> {
        Ok(ListDocumentSeriesRequestFilterItem {
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
