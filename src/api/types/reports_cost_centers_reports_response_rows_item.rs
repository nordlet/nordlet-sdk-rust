pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CostCentersReportsResponseRowsItem {
    #[serde(rename = "costCenterId")]
    #[serde(default)]
    pub cost_center_id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub income: String,
    #[serde(default)]
    pub expenses: String,
    #[serde(default)]
    pub result: String,
}

impl CostCentersReportsResponseRowsItem {
    pub fn builder() -> CostCentersReportsResponseRowsItemBuilder {
        <CostCentersReportsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CostCentersReportsResponseRowsItemBuilder {
    cost_center_id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    income: Option<String>,
    expenses: Option<String>,
    result: Option<String>,
}

impl CostCentersReportsResponseRowsItemBuilder {
    pub fn cost_center_id(mut self, value: impl Into<String>) -> Self {
        self.cost_center_id = Some(value.into());
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

    pub fn income(mut self, value: impl Into<String>) -> Self {
        self.income = Some(value.into());
        self
    }

    pub fn expenses(mut self, value: impl Into<String>) -> Self {
        self.expenses = Some(value.into());
        self
    }

    pub fn result(mut self, value: impl Into<String>) -> Self {
        self.result = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CostCentersReportsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cost_center_id`](CostCentersReportsResponseRowsItemBuilder::cost_center_id)
    /// - [`code`](CostCentersReportsResponseRowsItemBuilder::code)
    /// - [`name`](CostCentersReportsResponseRowsItemBuilder::name)
    /// - [`income`](CostCentersReportsResponseRowsItemBuilder::income)
    /// - [`expenses`](CostCentersReportsResponseRowsItemBuilder::expenses)
    /// - [`result`](CostCentersReportsResponseRowsItemBuilder::result)
    pub fn build(self) -> Result<CostCentersReportsResponseRowsItem, BuildError> {
        Ok(CostCentersReportsResponseRowsItem {
            cost_center_id: self
                .cost_center_id
                .ok_or_else(|| BuildError::missing_field("cost_center_id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            income: self
                .income
                .ok_or_else(|| BuildError::missing_field("income"))?,
            expenses: self
                .expenses
                .ok_or_else(|| BuildError::missing_field("expenses"))?,
            result: self
                .result
                .ok_or_else(|| BuildError::missing_field("result"))?,
        })
    }
}
