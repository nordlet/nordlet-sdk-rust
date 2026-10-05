pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TypesCreateAgreementsRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl TypesCreateAgreementsRequest {
    pub fn builder() -> TypesCreateAgreementsRequestBuilder {
        <TypesCreateAgreementsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TypesCreateAgreementsRequestBuilder {
    code: Option<String>,
    name: Option<String>,
}

impl TypesCreateAgreementsRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TypesCreateAgreementsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](TypesCreateAgreementsRequestBuilder::code)
    /// - [`name`](TypesCreateAgreementsRequestBuilder::name)
    pub fn build(self) -> Result<TypesCreateAgreementsRequest, BuildError> {
        Ok(TypesCreateAgreementsRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
