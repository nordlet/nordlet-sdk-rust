pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1PosDevicesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1PosDevicesListRequestFilterItemOp,
    pub value: PostV1PosDevicesListRequestFilterItemValue,
}

impl PostV1PosDevicesListRequestFilterItem {
    pub fn builder() -> PostV1PosDevicesListRequestFilterItemBuilder {
        <PostV1PosDevicesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PosDevicesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1PosDevicesListRequestFilterItemOp>,
    value: Option<PostV1PosDevicesListRequestFilterItemValue>,
}

impl PostV1PosDevicesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1PosDevicesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1PosDevicesListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PosDevicesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1PosDevicesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1PosDevicesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1PosDevicesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1PosDevicesListRequestFilterItem, BuildError> {
        Ok(PostV1PosDevicesListRequestFilterItem {
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
