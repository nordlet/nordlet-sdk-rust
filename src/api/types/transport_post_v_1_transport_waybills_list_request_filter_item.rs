pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1TransportWaybillsListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1TransportWaybillsListRequestFilterItemOp,
    pub value: PostV1TransportWaybillsListRequestFilterItemValue,
}

impl PostV1TransportWaybillsListRequestFilterItem {
    pub fn builder() -> PostV1TransportWaybillsListRequestFilterItemBuilder {
        <PostV1TransportWaybillsListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1TransportWaybillsListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1TransportWaybillsListRequestFilterItemOp>,
    value: Option<PostV1TransportWaybillsListRequestFilterItemValue>,
}

impl PostV1TransportWaybillsListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1TransportWaybillsListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: PostV1TransportWaybillsListRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1TransportWaybillsListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1TransportWaybillsListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1TransportWaybillsListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1TransportWaybillsListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1TransportWaybillsListRequestFilterItem, BuildError> {
        Ok(PostV1TransportWaybillsListRequestFilterItem {
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
