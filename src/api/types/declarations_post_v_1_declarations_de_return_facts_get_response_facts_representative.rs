pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentative {
    pub role: PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentativeRole,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub street: String,
    #[serde(rename = "houseNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub house_number: Option<String>,
    #[serde(rename = "postalCode")]
    #[serde(default)]
    pub postal_code: String,
    #[serde(default)]
    pub city: String,
}

impl PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentative {
    pub fn builder() -> PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentativeBuilder {
        <PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentativeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentativeBuilder {
    role: Option<PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentativeRole>,
    name: Option<String>,
    street: Option<String>,
    house_number: Option<String>,
    postal_code: Option<String>,
    city: Option<String>,
}

impl PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentativeBuilder {
    pub fn role(
        mut self,
        value: PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentativeRole,
    ) -> Self {
        self.role = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn street(mut self, value: impl Into<String>) -> Self {
        self.street = Some(value.into());
        self
    }

    pub fn house_number(mut self, value: impl Into<String>) -> Self {
        self.house_number = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentative`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentativeBuilder::role)
    /// - [`name`](PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentativeBuilder::name)
    /// - [`street`](PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentativeBuilder::street)
    /// - [`postal_code`](PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentativeBuilder::postal_code)
    /// - [`city`](PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentativeBuilder::city)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentative, BuildError> {
        Ok(
            PostV1DeclarationsDeReturnFactsGetResponseFactsRepresentative {
                role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
                street: self
                    .street
                    .ok_or_else(|| BuildError::missing_field("street"))?,
                house_number: self.house_number,
                postal_code: self
                    .postal_code
                    .ok_or_else(|| BuildError::missing_field("postal_code"))?,
                city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            },
        )
    }
}
