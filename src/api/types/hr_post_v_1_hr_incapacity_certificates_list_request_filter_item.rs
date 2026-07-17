pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1HrIncapacityCertificatesListRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: PostV1HrIncapacityCertificatesListRequestFilterItemOp,
    pub value: PostV1HrIncapacityCertificatesListRequestFilterItemValue,
}

impl PostV1HrIncapacityCertificatesListRequestFilterItem {
    pub fn builder() -> PostV1HrIncapacityCertificatesListRequestFilterItemBuilder {
        <PostV1HrIncapacityCertificatesListRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1HrIncapacityCertificatesListRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<PostV1HrIncapacityCertificatesListRequestFilterItemOp>,
    value: Option<PostV1HrIncapacityCertificatesListRequestFilterItemValue>,
}

impl PostV1HrIncapacityCertificatesListRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: PostV1HrIncapacityCertificatesListRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(
        mut self,
        value: PostV1HrIncapacityCertificatesListRequestFilterItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1HrIncapacityCertificatesListRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PostV1HrIncapacityCertificatesListRequestFilterItemBuilder::field)
    /// - [`op`](PostV1HrIncapacityCertificatesListRequestFilterItemBuilder::op)
    /// - [`value`](PostV1HrIncapacityCertificatesListRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<PostV1HrIncapacityCertificatesListRequestFilterItem, BuildError> {
        Ok(PostV1HrIncapacityCertificatesListRequestFilterItem {
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
