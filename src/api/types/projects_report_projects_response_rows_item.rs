pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReportProjectsResponseRowsItem {
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub name: String,
    pub status: ReportProjectsResponseRowsItemStatus,
    #[serde(default)]
    pub revenue: String,
    #[serde(default)]
    pub costs: String,
    #[serde(default)]
    pub profit: String,
    #[serde(rename = "totalHours")]
    #[serde(default)]
    pub total_hours: String,
    #[serde(rename = "billableHours")]
    #[serde(default)]
    pub billable_hours: String,
    #[serde(rename = "billedHours")]
    #[serde(default)]
    pub billed_hours: String,
    #[serde(rename = "unbilledHours")]
    #[serde(default)]
    pub unbilled_hours: String,
    #[serde(rename = "unbilledAmount")]
    #[serde(default)]
    pub unbilled_amount: String,
}

impl ReportProjectsResponseRowsItem {
    pub fn builder() -> ReportProjectsResponseRowsItemBuilder {
        <ReportProjectsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportProjectsResponseRowsItemBuilder {
    project_id: Option<String>,
    code: Option<String>,
    name: Option<String>,
    status: Option<ReportProjectsResponseRowsItemStatus>,
    revenue: Option<String>,
    costs: Option<String>,
    profit: Option<String>,
    total_hours: Option<String>,
    billable_hours: Option<String>,
    billed_hours: Option<String>,
    unbilled_hours: Option<String>,
    unbilled_amount: Option<String>,
}

impl ReportProjectsResponseRowsItemBuilder {
    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
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

    pub fn status(mut self, value: ReportProjectsResponseRowsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn revenue(mut self, value: impl Into<String>) -> Self {
        self.revenue = Some(value.into());
        self
    }

    pub fn costs(mut self, value: impl Into<String>) -> Self {
        self.costs = Some(value.into());
        self
    }

    pub fn profit(mut self, value: impl Into<String>) -> Self {
        self.profit = Some(value.into());
        self
    }

    pub fn total_hours(mut self, value: impl Into<String>) -> Self {
        self.total_hours = Some(value.into());
        self
    }

    pub fn billable_hours(mut self, value: impl Into<String>) -> Self {
        self.billable_hours = Some(value.into());
        self
    }

    pub fn billed_hours(mut self, value: impl Into<String>) -> Self {
        self.billed_hours = Some(value.into());
        self
    }

    pub fn unbilled_hours(mut self, value: impl Into<String>) -> Self {
        self.unbilled_hours = Some(value.into());
        self
    }

    pub fn unbilled_amount(mut self, value: impl Into<String>) -> Self {
        self.unbilled_amount = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReportProjectsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project_id`](ReportProjectsResponseRowsItemBuilder::project_id)
    /// - [`code`](ReportProjectsResponseRowsItemBuilder::code)
    /// - [`name`](ReportProjectsResponseRowsItemBuilder::name)
    /// - [`status`](ReportProjectsResponseRowsItemBuilder::status)
    /// - [`revenue`](ReportProjectsResponseRowsItemBuilder::revenue)
    /// - [`costs`](ReportProjectsResponseRowsItemBuilder::costs)
    /// - [`profit`](ReportProjectsResponseRowsItemBuilder::profit)
    /// - [`total_hours`](ReportProjectsResponseRowsItemBuilder::total_hours)
    /// - [`billable_hours`](ReportProjectsResponseRowsItemBuilder::billable_hours)
    /// - [`billed_hours`](ReportProjectsResponseRowsItemBuilder::billed_hours)
    /// - [`unbilled_hours`](ReportProjectsResponseRowsItemBuilder::unbilled_hours)
    /// - [`unbilled_amount`](ReportProjectsResponseRowsItemBuilder::unbilled_amount)
    pub fn build(self) -> Result<ReportProjectsResponseRowsItem, BuildError> {
        Ok(ReportProjectsResponseRowsItem {
            project_id: self
                .project_id
                .ok_or_else(|| BuildError::missing_field("project_id"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            revenue: self
                .revenue
                .ok_or_else(|| BuildError::missing_field("revenue"))?,
            costs: self
                .costs
                .ok_or_else(|| BuildError::missing_field("costs"))?,
            profit: self
                .profit
                .ok_or_else(|| BuildError::missing_field("profit"))?,
            total_hours: self
                .total_hours
                .ok_or_else(|| BuildError::missing_field("total_hours"))?,
            billable_hours: self
                .billable_hours
                .ok_or_else(|| BuildError::missing_field("billable_hours"))?,
            billed_hours: self
                .billed_hours
                .ok_or_else(|| BuildError::missing_field("billed_hours"))?,
            unbilled_hours: self
                .unbilled_hours
                .ok_or_else(|| BuildError::missing_field("unbilled_hours"))?,
            unbilled_amount: self
                .unbilled_amount
                .ok_or_else(|| BuildError::missing_field("unbilled_amount"))?,
        })
    }
}
