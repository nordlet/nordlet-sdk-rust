pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SeriesListReferenceRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: SeriesListReferenceRequestFilterItemOp,
    pub value: SeriesListReferenceRequestFilterItemValue,
}

impl SeriesListReferenceRequestFilterItem {
    pub fn builder() -> SeriesListReferenceRequestFilterItemBuilder {
        <SeriesListReferenceRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SeriesListReferenceRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<SeriesListReferenceRequestFilterItemOp>,
    value: Option<SeriesListReferenceRequestFilterItemValue>,
}

impl SeriesListReferenceRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: SeriesListReferenceRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: SeriesListReferenceRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SeriesListReferenceRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](SeriesListReferenceRequestFilterItemBuilder::field)
    /// - [`op`](SeriesListReferenceRequestFilterItemBuilder::op)
    /// - [`value`](SeriesListReferenceRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<SeriesListReferenceRequestFilterItem, BuildError> {
        Ok(SeriesListReferenceRequestFilterItem {
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
