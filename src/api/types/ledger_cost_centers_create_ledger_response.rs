pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCentersCreateLedgerResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl CostCentersCreateLedgerResponse {
    pub fn builder() -> CostCentersCreateLedgerResponseBuilder {
        <CostCentersCreateLedgerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCentersCreateLedgerResponseBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    group_id: Option<String>,
    group_name: Option<String>,
    is_active: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl CostCentersCreateLedgerResponseBuilder {
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

    pub fn group_id(mut self, value: impl Into<String>) -> Self {
        self.group_id = Some(value.into());
        self
    }

    pub fn group_name(mut self, value: impl Into<String>) -> Self {
        self.group_name = Some(value.into());
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CostCentersCreateLedgerResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CostCentersCreateLedgerResponseBuilder::id)
    /// - [`code`](CostCentersCreateLedgerResponseBuilder::code)
    /// - [`name`](CostCentersCreateLedgerResponseBuilder::name)
    /// - [`is_active`](CostCentersCreateLedgerResponseBuilder::is_active)
    /// - [`created_at`](CostCentersCreateLedgerResponseBuilder::created_at)
    pub fn build(self) -> Result<CostCentersCreateLedgerResponse, BuildError> {
        Ok(CostCentersCreateLedgerResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            group_id: self.group_id,
            group_name: self.group_name,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
