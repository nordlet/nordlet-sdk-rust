pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddressesListPartnersRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: AddressesListPartnersRequestFilterItemOp,
    pub value: AddressesListPartnersRequestFilterItemValue,
}

impl AddressesListPartnersRequestFilterItem {
    pub fn builder() -> AddressesListPartnersRequestFilterItemBuilder {
        <AddressesListPartnersRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddressesListPartnersRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<AddressesListPartnersRequestFilterItemOp>,
    value: Option<AddressesListPartnersRequestFilterItemValue>,
}

impl AddressesListPartnersRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: AddressesListPartnersRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: AddressesListPartnersRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddressesListPartnersRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](AddressesListPartnersRequestFilterItemBuilder::field)
    /// - [`op`](AddressesListPartnersRequestFilterItemBuilder::op)
    /// - [`value`](AddressesListPartnersRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<AddressesListPartnersRequestFilterItem, BuildError> {
        Ok(AddressesListPartnersRequestFilterItem {
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
