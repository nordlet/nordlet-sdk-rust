pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MeAccountResponseCompaniesItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "vatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_code: Option<String>,
    #[serde(default)]
    pub role: String,
    #[serde(rename = "isSandbox")]
    #[serde(default)]
    pub is_sandbox: bool,
    pub status: MeAccountResponseCompaniesItemStatus,
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub deleted_at: Option<DateTime<FixedOffset>>,
}

impl MeAccountResponseCompaniesItem {
    pub fn builder() -> MeAccountResponseCompaniesItemBuilder {
        <MeAccountResponseCompaniesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeAccountResponseCompaniesItemBuilder {
    id: Option<String>,
    name: Option<String>,
    code: Option<String>,
    vat_code: Option<String>,
    role: Option<String>,
    is_sandbox: Option<bool>,
    status: Option<MeAccountResponseCompaniesItemStatus>,
    deleted_at: Option<DateTime<FixedOffset>>,
}

impl MeAccountResponseCompaniesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn vat_code(mut self, value: impl Into<String>) -> Self {
        self.vat_code = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn is_sandbox(mut self, value: bool) -> Self {
        self.is_sandbox = Some(value);
        self
    }

    pub fn status(mut self, value: MeAccountResponseCompaniesItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeAccountResponseCompaniesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MeAccountResponseCompaniesItemBuilder::id)
    /// - [`name`](MeAccountResponseCompaniesItemBuilder::name)
    /// - [`role`](MeAccountResponseCompaniesItemBuilder::role)
    /// - [`is_sandbox`](MeAccountResponseCompaniesItemBuilder::is_sandbox)
    /// - [`status`](MeAccountResponseCompaniesItemBuilder::status)
    pub fn build(self) -> Result<MeAccountResponseCompaniesItem, BuildError> {
        Ok(MeAccountResponseCompaniesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            code: self.code,
            vat_code: self.vat_code,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            is_sandbox: self
                .is_sandbox
                .ok_or_else(|| BuildError::missing_field("is_sandbox"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            deleted_at: self.deleted_at,
        })
    }
}
