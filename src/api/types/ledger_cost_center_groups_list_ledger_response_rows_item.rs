pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCenterGroupsListLedgerResponseRowsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl CostCenterGroupsListLedgerResponseRowsItem {
    pub fn builder() -> CostCenterGroupsListLedgerResponseRowsItemBuilder {
        <CostCenterGroupsListLedgerResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCenterGroupsListLedgerResponseRowsItemBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl CostCenterGroupsListLedgerResponseRowsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CostCenterGroupsListLedgerResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CostCenterGroupsListLedgerResponseRowsItemBuilder::id)
    /// - [`code`](CostCenterGroupsListLedgerResponseRowsItemBuilder::code)
    /// - [`name`](CostCenterGroupsListLedgerResponseRowsItemBuilder::name)
    /// - [`created_at`](CostCenterGroupsListLedgerResponseRowsItemBuilder::created_at)
    pub fn build(self) -> Result<CostCenterGroupsListLedgerResponseRowsItem, BuildError> {
        Ok(CostCenterGroupsListLedgerResponseRowsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
