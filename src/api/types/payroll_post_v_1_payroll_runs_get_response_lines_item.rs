pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsGetResponseLinesItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "contractId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_id: Option<String>,
    #[serde(rename = "employeeName")]
    #[serde(default)]
    pub employee_name: String,
    #[serde(default)]
    pub gross: String,
    #[serde(default)]
    pub additions: Vec<PostV1PayrollRunsGetResponseLinesItemAdditionsItem>,
    #[serde(default)]
    pub deductions: Vec<PostV1PayrollRunsGetResponseLinesItemDeductionsItem>,
    #[serde(rename = "taxableBase")]
    #[serde(default)]
    pub taxable_base: String,
    #[serde(default)]
    pub npd: String,
    #[serde(default)]
    pub gpm: String,
    #[serde(rename = "sodraEmployee")]
    #[serde(default)]
    pub sodra_employee: String,
    #[serde(rename = "sodraEmployer")]
    #[serde(default)]
    pub sodra_employer: String,
    #[serde(default)]
    pub net: String,
}

impl PostV1PayrollRunsGetResponseLinesItem {
    pub fn builder() -> PostV1PayrollRunsGetResponseLinesItemBuilder {
        <PostV1PayrollRunsGetResponseLinesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsGetResponseLinesItemBuilder {
    id: Option<String>,
    employee_id: Option<String>,
    contract_id: Option<String>,
    employee_name: Option<String>,
    gross: Option<String>,
    additions: Option<Vec<PostV1PayrollRunsGetResponseLinesItemAdditionsItem>>,
    deductions: Option<Vec<PostV1PayrollRunsGetResponseLinesItemDeductionsItem>>,
    taxable_base: Option<String>,
    npd: Option<String>,
    gpm: Option<String>,
    sodra_employee: Option<String>,
    sodra_employer: Option<String>,
    net: Option<String>,
}

impl PostV1PayrollRunsGetResponseLinesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn contract_id(mut self, value: impl Into<String>) -> Self {
        self.contract_id = Some(value.into());
        self
    }

    pub fn employee_name(mut self, value: impl Into<String>) -> Self {
        self.employee_name = Some(value.into());
        self
    }

    pub fn gross(mut self, value: impl Into<String>) -> Self {
        self.gross = Some(value.into());
        self
    }

    pub fn additions(
        mut self,
        value: Vec<PostV1PayrollRunsGetResponseLinesItemAdditionsItem>,
    ) -> Self {
        self.additions = Some(value);
        self
    }

    pub fn deductions(
        mut self,
        value: Vec<PostV1PayrollRunsGetResponseLinesItemDeductionsItem>,
    ) -> Self {
        self.deductions = Some(value);
        self
    }

    pub fn taxable_base(mut self, value: impl Into<String>) -> Self {
        self.taxable_base = Some(value.into());
        self
    }

    pub fn npd(mut self, value: impl Into<String>) -> Self {
        self.npd = Some(value.into());
        self
    }

    pub fn gpm(mut self, value: impl Into<String>) -> Self {
        self.gpm = Some(value.into());
        self
    }

    pub fn sodra_employee(mut self, value: impl Into<String>) -> Self {
        self.sodra_employee = Some(value.into());
        self
    }

    pub fn sodra_employer(mut self, value: impl Into<String>) -> Self {
        self.sodra_employer = Some(value.into());
        self
    }

    pub fn net(mut self, value: impl Into<String>) -> Self {
        self.net = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsGetResponseLinesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PayrollRunsGetResponseLinesItemBuilder::id)
    /// - [`employee_id`](PostV1PayrollRunsGetResponseLinesItemBuilder::employee_id)
    /// - [`employee_name`](PostV1PayrollRunsGetResponseLinesItemBuilder::employee_name)
    /// - [`gross`](PostV1PayrollRunsGetResponseLinesItemBuilder::gross)
    /// - [`additions`](PostV1PayrollRunsGetResponseLinesItemBuilder::additions)
    /// - [`deductions`](PostV1PayrollRunsGetResponseLinesItemBuilder::deductions)
    /// - [`taxable_base`](PostV1PayrollRunsGetResponseLinesItemBuilder::taxable_base)
    /// - [`npd`](PostV1PayrollRunsGetResponseLinesItemBuilder::npd)
    /// - [`gpm`](PostV1PayrollRunsGetResponseLinesItemBuilder::gpm)
    /// - [`sodra_employee`](PostV1PayrollRunsGetResponseLinesItemBuilder::sodra_employee)
    /// - [`sodra_employer`](PostV1PayrollRunsGetResponseLinesItemBuilder::sodra_employer)
    /// - [`net`](PostV1PayrollRunsGetResponseLinesItemBuilder::net)
    pub fn build(self) -> Result<PostV1PayrollRunsGetResponseLinesItem, BuildError> {
        Ok(PostV1PayrollRunsGetResponseLinesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            contract_id: self.contract_id,
            employee_name: self
                .employee_name
                .ok_or_else(|| BuildError::missing_field("employee_name"))?,
            gross: self
                .gross
                .ok_or_else(|| BuildError::missing_field("gross"))?,
            additions: self
                .additions
                .ok_or_else(|| BuildError::missing_field("additions"))?,
            deductions: self
                .deductions
                .ok_or_else(|| BuildError::missing_field("deductions"))?,
            taxable_base: self
                .taxable_base
                .ok_or_else(|| BuildError::missing_field("taxable_base"))?,
            npd: self.npd.ok_or_else(|| BuildError::missing_field("npd"))?,
            gpm: self.gpm.ok_or_else(|| BuildError::missing_field("gpm"))?,
            sodra_employee: self
                .sodra_employee
                .ok_or_else(|| BuildError::missing_field("sodra_employee"))?,
            sodra_employer: self
                .sodra_employer
                .ok_or_else(|| BuildError::missing_field("sodra_employer"))?,
            net: self.net.ok_or_else(|| BuildError::missing_field("net"))?,
        })
    }
}
