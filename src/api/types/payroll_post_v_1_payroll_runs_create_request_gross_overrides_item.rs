pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsCreateRequestGrossOverridesItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub gross: String,
}

impl PostV1PayrollRunsCreateRequestGrossOverridesItem {
    pub fn builder() -> PostV1PayrollRunsCreateRequestGrossOverridesItemBuilder {
        <PostV1PayrollRunsCreateRequestGrossOverridesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsCreateRequestGrossOverridesItemBuilder {
    employee_id: Option<String>,
    gross: Option<String>,
}

impl PostV1PayrollRunsCreateRequestGrossOverridesItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn gross(mut self, value: impl Into<String>) -> Self {
        self.gross = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsCreateRequestGrossOverridesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1PayrollRunsCreateRequestGrossOverridesItemBuilder::employee_id)
    /// - [`gross`](PostV1PayrollRunsCreateRequestGrossOverridesItemBuilder::gross)
    pub fn build(self) -> Result<PostV1PayrollRunsCreateRequestGrossOverridesItem, BuildError> {
        Ok(PostV1PayrollRunsCreateRequestGrossOverridesItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
        })
    }
}
