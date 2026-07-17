pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1PartnersBankAccountsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1PartnersBankAccountsListRequestFilterItemOp,
    pub value: PostV1PartnersBankAccountsListRequestFilterItemValue,
}

impl PostV1PartnersBankAccountsListRequestFilterItem {
    pub fn builder() -> PostV1PartnersBankAccountsListRequestFilterItemBuilder {
        <PostV1PartnersBankAccountsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersBankAccountsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1PartnersBankAccountsListRequestFilterItemOp>,
    value: Option<PostV1PartnersBankAccountsListRequestFilterItemValue>,
}

impl PostV1PartnersBankAccountsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1PartnersBankAccountsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1PartnersBankAccountsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersBankAccountsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PartnersBankAccountsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1PartnersBankAccountsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1PartnersBankAccountsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1PartnersBankAccountsListRequestFilterItem, BuildError> {
        Ok(PostV1PartnersBankAccountsListRequestFilterItem {
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
