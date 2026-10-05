pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BanksListReferenceRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: BanksListReferenceRequestFilterItemOp,
    pub value: BanksListReferenceRequestFilterItemValue,
}

impl BanksListReferenceRequestFilterItem {
    pub fn builder() -> BanksListReferenceRequestFilterItemBuilder {
        <BanksListReferenceRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BanksListReferenceRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<BanksListReferenceRequestFilterItemOp>,
    value: Option<BanksListReferenceRequestFilterItemValue>,
}

impl BanksListReferenceRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: BanksListReferenceRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: BanksListReferenceRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BanksListReferenceRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](BanksListReferenceRequestFilterItemBuilder::field)
    /// - [`op`](BanksListReferenceRequestFilterItemBuilder::op)
    /// - [`value`](BanksListReferenceRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<BanksListReferenceRequestFilterItem, BuildError> {
        Ok(BanksListReferenceRequestFilterItem {
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
