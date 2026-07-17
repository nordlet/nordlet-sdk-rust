pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1PartnersAddressesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1PartnersAddressesListRequestFilterItemOp,
    pub value: PostV1PartnersAddressesListRequestFilterItemValue,
}

impl PostV1PartnersAddressesListRequestFilterItem {
    pub fn builder() -> PostV1PartnersAddressesListRequestFilterItemBuilder {
        <PostV1PartnersAddressesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersAddressesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1PartnersAddressesListRequestFilterItemOp>,
    value: Option<PostV1PartnersAddressesListRequestFilterItemValue>,
}

impl PostV1PartnersAddressesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1PartnersAddressesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1PartnersAddressesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersAddressesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PartnersAddressesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1PartnersAddressesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1PartnersAddressesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1PartnersAddressesListRequestFilterItem, BuildError> {
        Ok(PostV1PartnersAddressesListRequestFilterItem {
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
