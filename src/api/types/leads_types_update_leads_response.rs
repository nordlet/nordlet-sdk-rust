pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TypesUpdateLeadsResponse {
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

impl TypesUpdateLeadsResponse {
    pub fn builder() -> TypesUpdateLeadsResponseBuilder {
        <TypesUpdateLeadsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesUpdateLeadsResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    is_active: Option<bool>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl TypesUpdateLeadsResponseBuilder {
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

    /// Consumes the builder and constructs a [`TypesUpdateLeadsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TypesUpdateLeadsResponseBuilder::id)
    /// - [`name`](TypesUpdateLeadsResponseBuilder::name)
    /// - [`is_active`](TypesUpdateLeadsResponseBuilder::is_active)
    /// - [`created_at`](TypesUpdateLeadsResponseBuilder::created_at)
    pub fn build(self) -> Result<TypesUpdateLeadsResponse, BuildError> {
        Ok(TypesUpdateLeadsResponse {
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
