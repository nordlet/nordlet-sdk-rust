pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessTripsListHrRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<BusinessTripsListHrRequestSortItemDir>,
}

impl BusinessTripsListHrRequestSortItem {
    pub fn builder() -> BusinessTripsListHrRequestSortItemBuilder {
        <BusinessTripsListHrRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessTripsListHrRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<BusinessTripsListHrRequestSortItemDir>,
}

impl BusinessTripsListHrRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: BusinessTripsListHrRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessTripsListHrRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](BusinessTripsListHrRequestSortItemBuilder::field)
    pub fn build(self) -> Result<BusinessTripsListHrRequestSortItem, BuildError> {
        Ok(BusinessTripsListHrRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
