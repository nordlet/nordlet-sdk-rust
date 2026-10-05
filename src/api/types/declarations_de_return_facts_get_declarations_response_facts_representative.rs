pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeReturnFactsGetDeclarationsResponseFactsRepresentative {
    pub role: DeReturnFactsGetDeclarationsResponseFactsRepresentativeRole,
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

impl DeReturnFactsGetDeclarationsResponseFactsRepresentative {
    pub fn builder() -> DeReturnFactsGetDeclarationsResponseFactsRepresentativeBuilder {
        <DeReturnFactsGetDeclarationsResponseFactsRepresentativeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsGetDeclarationsResponseFactsRepresentativeBuilder {
    role: Option<DeReturnFactsGetDeclarationsResponseFactsRepresentativeRole>,
    name: Option<String>,
    street: Option<String>,
    house_number: Option<String>,
    postal_code: Option<String>,
    city: Option<String>,
}

impl DeReturnFactsGetDeclarationsResponseFactsRepresentativeBuilder {
    pub fn role(
        mut self,
        value: DeReturnFactsGetDeclarationsResponseFactsRepresentativeRole,
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

    /// Consumes the builder and constructs a [`DeReturnFactsGetDeclarationsResponseFactsRepresentative`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](DeReturnFactsGetDeclarationsResponseFactsRepresentativeBuilder::role)
    /// - [`name`](DeReturnFactsGetDeclarationsResponseFactsRepresentativeBuilder::name)
    /// - [`street`](DeReturnFactsGetDeclarationsResponseFactsRepresentativeBuilder::street)
    /// - [`postal_code`](DeReturnFactsGetDeclarationsResponseFactsRepresentativeBuilder::postal_code)
    /// - [`city`](DeReturnFactsGetDeclarationsResponseFactsRepresentativeBuilder::city)
    pub fn build(
        self,
    ) -> Result<DeReturnFactsGetDeclarationsResponseFactsRepresentative, BuildError> {
        Ok(DeReturnFactsGetDeclarationsResponseFactsRepresentative {
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
        })
    }
}
