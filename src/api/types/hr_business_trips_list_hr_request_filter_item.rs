pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessTripsListHrRequestFilterItem {
    #[serde(default)]
    pub field: String,
    pub op: BusinessTripsListHrRequestFilterItemOp,
    pub value: BusinessTripsListHrRequestFilterItemValue,
}

impl BusinessTripsListHrRequestFilterItem {
    pub fn builder() -> BusinessTripsListHrRequestFilterItemBuilder {
        <BusinessTripsListHrRequestFilterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessTripsListHrRequestFilterItemBuilder {
    field: Option<String>,
    op: Option<BusinessTripsListHrRequestFilterItemOp>,
    value: Option<BusinessTripsListHrRequestFilterItemValue>,
}

impl BusinessTripsListHrRequestFilterItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn op(mut self, value: BusinessTripsListHrRequestFilterItemOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn value(mut self, value: BusinessTripsListHrRequestFilterItemValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessTripsListHrRequestFilterItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](BusinessTripsListHrRequestFilterItemBuilder::field)
    /// - [`op`](BusinessTripsListHrRequestFilterItemBuilder::op)
    /// - [`value`](BusinessTripsListHrRequestFilterItemBuilder::value)
    pub fn build(self) -> Result<BusinessTripsListHrRequestFilterItem, BuildError> {
        Ok(BusinessTripsListHrRequestFilterItem {
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
