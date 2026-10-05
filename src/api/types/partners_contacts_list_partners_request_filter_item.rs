pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContactsListPartnersRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: ContactsListPartnersRequestFilterItemOp,
    pub value: ContactsListPartnersRequestFilterItemValue,
}

impl ContactsListPartnersRequestFilterItem {
    pub fn builder() -> ContactsListPartnersRequestFilterItemBuilder {
        <ContactsListPartnersRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContactsListPartnersRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<ContactsListPartnersRequestFilterItemOp>,
    value: Option<ContactsListPartnersRequestFilterItemValue>,
}

impl ContactsListPartnersRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: ContactsListPartnersRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: ContactsListPartnersRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContactsListPartnersRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ContactsListPartnersRequestFilterItemBuilder::field)
    /// - [`op`](ContactsListPartnersRequestFilterItemBuilder::op)
    /// - [`value`](ContactsListPartnersRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<ContactsListPartnersRequestFilterItem, BuildError> {
        Ok(ContactsListPartnersRequestFilterItem {
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
