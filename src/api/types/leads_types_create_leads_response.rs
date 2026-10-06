pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TypesCreateLeadsResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "isActive")]
    #[serde(default)]
    pub is_active: bool,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl TypesCreateLeadsResponse {
    pub fn builder() -> TypesCreateLeadsResponseBuilder {
        <TypesCreateLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesCreateLeadsResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    is_active: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl TypesCreateLeadsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    /// Consumes the builder and constructs a [`TypesCreateLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TypesCreateLeadsResponseBuilder::id)
    /// - [`name`](TypesCreateLeadsResponseBuilder::name)
    /// - [`is_active`](TypesCreateLeadsResponseBuilder::is_active)
    /// - [`created_at`](TypesCreateLeadsResponseBuilder::created_at)
    pub fn build(self) -> Result<TypesCreateLeadsResponse, BuildError> {
        Ok(TypesCreateLeadsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_active: self
                .is_active
                .ok_or_else(|| BuildError::missing_field("is_active"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
