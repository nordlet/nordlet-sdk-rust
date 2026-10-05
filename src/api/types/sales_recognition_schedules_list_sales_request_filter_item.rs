pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecognitionSchedulesListSalesRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: RecognitionSchedulesListSalesRequestFilterItemOp,
    pub value: RecognitionSchedulesListSalesRequestFilterItemValue,
}

impl RecognitionSchedulesListSalesRequestFilterItem {
    pub fn builder() -> RecognitionSchedulesListSalesRequestFilterItemBuilder {
        <RecognitionSchedulesListSalesRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionSchedulesListSalesRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<RecognitionSchedulesListSalesRequestFilterItemOp>,
    value: Option<RecognitionSchedulesListSalesRequestFilterItemValue>,
}

impl RecognitionSchedulesListSalesRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: RecognitionSchedulesListSalesRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: RecognitionSchedulesListSalesRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecognitionSchedulesListSalesRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RecognitionSchedulesListSalesRequestFilterItemBuilder::field)
    /// - [`op`](RecognitionSchedulesListSalesRequestFilterItemBuilder::op)
    /// - [`value`](RecognitionSchedulesListSalesRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<RecognitionSchedulesListSalesRequestFilterItem, BuildError> {
        Ok(RecognitionSchedulesListSalesRequestFilterItem {
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
