pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCenterActivityReportsResponseCostCenter {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
}

impl CostCenterActivityReportsResponseCostCenter {
    pub fn builder() -> CostCenterActivityReportsResponseCostCenterBuilder {
        <CostCenterActivityReportsResponseCostCenterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCenterActivityReportsResponseCostCenterBuilder {
    id: Option<String>,
    code: Option<String>,
    name: Option<String>,
}

impl CostCenterActivityReportsResponseCostCenterBuilder {
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

    /// Consumes the builder and constructs a [`CostCenterActivityReportsResponseCostCenter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CostCenterActivityReportsResponseCostCenterBuilder::id)
    /// - [`code`](CostCenterActivityReportsResponseCostCenterBuilder::code)
    /// - [`name`](CostCenterActivityReportsResponseCostCenterBuilder::name)
    pub fn build(self) -> Result<CostCenterActivityReportsResponseCostCenter, BuildError> {
        Ok(CostCenterActivityReportsResponseCostCenter {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
