pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1BankImportTemplatesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1BankImportTemplatesListRequestFilterItemOp,
    pub value: PostV1BankImportTemplatesListRequestFilterItemValue,
}

impl PostV1BankImportTemplatesListRequestFilterItem {
    pub fn builder() -> PostV1BankImportTemplatesListRequestFilterItemBuilder {
        <PostV1BankImportTemplatesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1BankImportTemplatesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1BankImportTemplatesListRequestFilterItemOp>,
    value: Option<PostV1BankImportTemplatesListRequestFilterItemValue>,
}

impl PostV1BankImportTemplatesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1BankImportTemplatesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1BankImportTemplatesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1BankImportTemplatesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1BankImportTemplatesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1BankImportTemplatesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1BankImportTemplatesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1BankImportTemplatesListRequestFilterItem, BuildError> {
        Ok(PostV1BankImportTemplatesListRequestFilterItem {
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
