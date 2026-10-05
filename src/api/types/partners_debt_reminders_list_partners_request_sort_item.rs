pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DebtRemindersListPartnersRequestSortItem {
    #[serde(default)]
    pub field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dir: Option<DebtRemindersListPartnersRequestSortItemDir>,
}

impl DebtRemindersListPartnersRequestSortItem {
    pub fn builder() -> DebtRemindersListPartnersRequestSortItemBuilder {
        <DebtRemindersListPartnersRequestSortItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtRemindersListPartnersRequestSortItemBuilder {
    field: Option<String>,
    dir: Option<DebtRemindersListPartnersRequestSortItemDir>,
}

impl DebtRemindersListPartnersRequestSortItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn dir(mut self, value: DebtRemindersListPartnersRequestSortItemDir) -> Self {
        self.dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DebtRemindersListPartnersRequestSortItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](DebtRemindersListPartnersRequestSortItemBuilder::field)
    pub fn build(self) -> Result<DebtRemindersListPartnersRequestSortItem, BuildError> {
        Ok(DebtRemindersListPartnersRequestSortItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            dir: self.dir,
        })
    }
}
