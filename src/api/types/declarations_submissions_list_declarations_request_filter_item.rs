pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmissionsListDeclarationsRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: SubmissionsListDeclarationsRequestFilterItemOp,
    pub value: SubmissionsListDeclarationsRequestFilterItemValue,
}

impl SubmissionsListDeclarationsRequestFilterItem {
    pub fn builder() -> SubmissionsListDeclarationsRequestFilterItemBuilder {
        <SubmissionsListDeclarationsRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmissionsListDeclarationsRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<SubmissionsListDeclarationsRequestFilterItemOp>,
    value: Option<SubmissionsListDeclarationsRequestFilterItemValue>,
}

impl SubmissionsListDeclarationsRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: SubmissionsListDeclarationsRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: SubmissionsListDeclarationsRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmissionsListDeclarationsRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](SubmissionsListDeclarationsRequestFilterItemBuilder::field)
    /// - [`op`](SubmissionsListDeclarationsRequestFilterItemBuilder::op)
    /// - [`value`](SubmissionsListDeclarationsRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<SubmissionsListDeclarationsRequestFilterItem, BuildError> {
        Ok(SubmissionsListDeclarationsRequestFilterItem {
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
