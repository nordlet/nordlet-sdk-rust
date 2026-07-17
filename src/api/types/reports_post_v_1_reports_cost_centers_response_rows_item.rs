pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ReportsCostCentersResponseRowsItem {
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

impl PostV1ReportsCostCentersResponseRowsItem {
    pub fn builder() -> PostV1ReportsCostCentersResponseRowsItemBuilder {
        <PostV1ReportsCostCentersResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ReportsCostCentersResponseRowsItemBuilder {
    cost_center_id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    income: Option<String>,
    expenses: Option<String>,
    result: Option<String>,
}

impl PostV1ReportsCostCentersResponseRowsItemBuilder {
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

    /// Consumes the builder and constructs a [`PostV1ReportsCostCentersResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cost_center_id`](PostV1ReportsCostCentersResponseRowsItemBuilder::cost_center_id)
    /// - [`code`](PostV1ReportsCostCentersResponseRowsItemBuilder::code)
    /// - [`name`](PostV1ReportsCostCentersResponseRowsItemBuilder::name)
    /// - [`income`](PostV1ReportsCostCentersResponseRowsItemBuilder::income)
    /// - [`expenses`](PostV1ReportsCostCentersResponseRowsItemBuilder::expenses)
    /// - [`result`](PostV1ReportsCostCentersResponseRowsItemBuilder::result)
    pub fn build(self) -> Result<PostV1ReportsCostCentersResponseRowsItem, BuildError> {
        Ok(PostV1ReportsCostCentersResponseRowsItem {
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
