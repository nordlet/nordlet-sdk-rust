pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WaybillsListTransportRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: WaybillsListTransportRequestFilterItemOp,
    pub value: WaybillsListTransportRequestFilterItemValue,
}

impl WaybillsListTransportRequestFilterItem {
    pub fn builder() -> WaybillsListTransportRequestFilterItemBuilder {
        <WaybillsListTransportRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaybillsListTransportRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<WaybillsListTransportRequestFilterItemOp>,
    value: Option<WaybillsListTransportRequestFilterItemValue>,
}

impl WaybillsListTransportRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: WaybillsListTransportRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: WaybillsListTransportRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WaybillsListTransportRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](WaybillsListTransportRequestFilterItemBuilder::field)
    /// - [`op`](WaybillsListTransportRequestFilterItemBuilder::op)
    /// - [`value`](WaybillsListTransportRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<WaybillsListTransportRequestFilterItem, BuildError> {
        Ok(WaybillsListTransportRequestFilterItem {
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
