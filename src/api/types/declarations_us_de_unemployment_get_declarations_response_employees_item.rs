pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsDeUnemploymentGetDeclarationsResponseEmployeesItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "socialSecurityNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_security_number: Option<String>,
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub wages: String,
}

impl UsDeUnemploymentGetDeclarationsResponseEmployeesItem {
    pub fn builder() -> UsDeUnemploymentGetDeclarationsResponseEmployeesItemBuilder {
        <UsDeUnemploymentGetDeclarationsResponseEmployeesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsDeUnemploymentGetDeclarationsResponseEmployeesItemBuilder {
    employee_id: Option<String>,
    social_security_number: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    wages: Option<String>,
}

impl UsDeUnemploymentGetDeclarationsResponseEmployeesItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn social_security_number(mut self, value: impl Into<String>) -> Self {
        self.social_security_number = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn wages(mut self, value: impl Into<String>) -> Self {
        self.wages = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsDeUnemploymentGetDeclarationsResponseEmployeesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](UsDeUnemploymentGetDeclarationsResponseEmployeesItemBuilder::employee_id)
    /// - [`first_name`](UsDeUnemploymentGetDeclarationsResponseEmployeesItemBuilder::first_name)
    /// - [`last_name`](UsDeUnemploymentGetDeclarationsResponseEmployeesItemBuilder::last_name)
    /// - [`wages`](UsDeUnemploymentGetDeclarationsResponseEmployeesItemBuilder::wages)
    pub fn build(self) -> Result<UsDeUnemploymentGetDeclarationsResponseEmployeesItem, BuildError> {
        Ok(UsDeUnemploymentGetDeclarationsResponseEmployeesItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            social_security_number: self.social_security_number,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            wages: self
                .wages
                .ok_or_else(|| BuildError::missing_field("wages"))?,
        })
    }
}
