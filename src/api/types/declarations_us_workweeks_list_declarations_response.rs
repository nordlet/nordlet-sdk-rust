pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsWorkweeksListDeclarationsResponse {
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub employees: Vec<UsWorkweeksListDeclarationsResponseEmployeesItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl UsWorkweeksListDeclarationsResponse {
    pub fn builder() -> UsWorkweeksListDeclarationsResponseBuilder {
        <UsWorkweeksListDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsWorkweeksListDeclarationsResponseBuilder {
    from: Option<String>,
    to: Option<String>,
    employees: Option<Vec<UsWorkweeksListDeclarationsResponseEmployeesItem>>,
    warnings: Option<Vec<String>>,
}

impl UsWorkweeksListDeclarationsResponseBuilder {
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn to(mut self, value: impl Into<String>) -> Self {
        self.to = Some(value.into());
        self
    }

    pub fn employees(
        mut self,
        value: Vec<UsWorkweeksListDeclarationsResponseEmployeesItem>,
    ) -> Self {
        self.employees = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsWorkweeksListDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](UsWorkweeksListDeclarationsResponseBuilder::from)
    /// - [`to`](UsWorkweeksListDeclarationsResponseBuilder::to)
    /// - [`employees`](UsWorkweeksListDeclarationsResponseBuilder::employees)
    /// - [`warnings`](UsWorkweeksListDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<UsWorkweeksListDeclarationsResponse, BuildError> {
        Ok(UsWorkweeksListDeclarationsResponse {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
            employees: self
                .employees
                .ok_or_else(|| BuildError::missing_field("employees"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
