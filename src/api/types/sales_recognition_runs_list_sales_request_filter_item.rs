pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecognitionRunsListSalesRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: RecognitionRunsListSalesRequestFilterItemOp,
    pub value: RecognitionRunsListSalesRequestFilterItemValue,
}

impl RecognitionRunsListSalesRequestFilterItem {
    pub fn builder() -> RecognitionRunsListSalesRequestFilterItemBuilder {
        <RecognitionRunsListSalesRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecognitionRunsListSalesRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<RecognitionRunsListSalesRequestFilterItemOp>,
    value: Option<RecognitionRunsListSalesRequestFilterItemValue>,
}

impl RecognitionRunsListSalesRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: RecognitionRunsListSalesRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: RecognitionRunsListSalesRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecognitionRunsListSalesRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](RecognitionRunsListSalesRequestFilterItemBuilder::field)
    /// - [`op`](RecognitionRunsListSalesRequestFilterItemBuilder::op)
    /// - [`value`](RecognitionRunsListSalesRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<RecognitionRunsListSalesRequestFilterItem, BuildError> {
        Ok(RecognitionRunsListSalesRequestFilterItem {
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
