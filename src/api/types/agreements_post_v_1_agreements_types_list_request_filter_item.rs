pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1AgreementsTypesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1AgreementsTypesListRequestFilterItemOp,
    pub value: PostV1AgreementsTypesListRequestFilterItemValue,
}

impl PostV1AgreementsTypesListRequestFilterItem {
    pub fn builder() -> PostV1AgreementsTypesListRequestFilterItemBuilder {
        <PostV1AgreementsTypesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1AgreementsTypesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1AgreementsTypesListRequestFilterItemOp>,
    value: Option<PostV1AgreementsTypesListRequestFilterItemValue>,
}

impl PostV1AgreementsTypesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1AgreementsTypesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1AgreementsTypesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1AgreementsTypesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1AgreementsTypesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1AgreementsTypesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1AgreementsTypesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1AgreementsTypesListRequestFilterItem, BuildError> {
        Ok(PostV1AgreementsTypesListRequestFilterItem {
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
