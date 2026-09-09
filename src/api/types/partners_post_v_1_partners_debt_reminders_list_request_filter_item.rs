pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1PartnersDebtRemindersListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1PartnersDebtRemindersListRequestFilterItemOp,
    pub value: PostV1PartnersDebtRemindersListRequestFilterItemValue,
}

impl PostV1PartnersDebtRemindersListRequestFilterItem {
    pub fn builder() -> PostV1PartnersDebtRemindersListRequestFilterItemBuilder {
        <PostV1PartnersDebtRemindersListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PartnersDebtRemindersListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1PartnersDebtRemindersListRequestFilterItemOp>,
    value: Option<PostV1PartnersDebtRemindersListRequestFilterItemValue>,
}

impl PostV1PartnersDebtRemindersListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1PartnersDebtRemindersListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1PartnersDebtRemindersListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PartnersDebtRemindersListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PartnersDebtRemindersListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1PartnersDebtRemindersListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1PartnersDebtRemindersListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1PartnersDebtRemindersListRequestFilterItem, BuildError> {
        Ok(PostV1PartnersDebtRemindersListRequestFilterItem {
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
