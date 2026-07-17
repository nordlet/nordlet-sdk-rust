pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1PayrollRunsCreateResponse {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub year: i64,
    #[serde(default)]
    pub month: i64,
    pub status: PostV1PayrollRunsCreateResponseStatus,
    #[serde(rename = "grossTotal")]
    #[serde(default)]
    pub gross_total: String,
    #[serde(rename = "npdTotal")]
    #[serde(default)]
    pub npd_total: String,
    #[serde(rename = "gpmTotal")]
    #[serde(default)]
    pub gpm_total: String,
    #[serde(rename = "sodraEmployeeTotal")]
    #[serde(default)]
    pub sodra_employee_total: String,
    #[serde(rename = "sodraEmployerTotal")]
    #[serde(default)]
    pub sodra_employer_total: String,
    #[serde(rename = "netTotal")]
    #[serde(default)]
    pub net_total: String,
    #[serde(rename = "journalTransactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "approvedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<String>,
    #[serde(default)]
    pub lines: Vec<PostV1PayrollRunsCreateResponseLinesItem>,
}

impl PostV1PayrollRunsCreateResponse {
    pub fn builder() -> PostV1PayrollRunsCreateResponseBuilder {
        <PostV1PayrollRunsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1PayrollRunsCreateResponseBuilder {
    id: Option<String>,
    year: Option<i64>,
    month: Option<i64>,
    status: Option<PostV1PayrollRunsCreateResponseStatus>,
    gross_total: Option<String>,
    npd_total: Option<String>,
    gpm_total: Option<String>,
    sodra_employee_total: Option<String>,
    sodra_employer_total: Option<String>,
    net_total: Option<String>,
    journal_transaction_id: Option<String>,
    notes: Option<String>,
    created_at: Option<String>,
    approved_at: Option<String>,
    lines: Option<Vec<PostV1PayrollRunsCreateResponseLinesItem>>,
}

impl PostV1PayrollRunsCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn month(mut self, value: i64) -> Self {
        self.month = Some(value);
        self
    }

    pub fn status(mut self, value: PostV1PayrollRunsCreateResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn gross_total(mut self, value: impl Into<String>) -> Self {
        self.gross_total = Some(value.into());
        self
    }

    pub fn npd_total(mut self, value: impl Into<String>) -> Self {
        self.npd_total = Some(value.into());
        self
    }

    pub fn gpm_total(mut self, value: impl Into<String>) -> Self {
        self.gpm_total = Some(value.into());
        self
    }

    pub fn sodra_employee_total(mut self, value: impl Into<String>) -> Self {
        self.sodra_employee_total = Some(value.into());
        self
    }

    pub fn sodra_employer_total(mut self, value: impl Into<String>) -> Self {
        self.sodra_employer_total = Some(value.into());
        self
    }

    pub fn net_total(mut self, value: impl Into<String>) -> Self {
        self.net_total = Some(value.into());
        self
    }

    pub fn journal_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.journal_transaction_id = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn approved_at(mut self, value: impl Into<String>) -> Self {
        self.approved_at = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<PostV1PayrollRunsCreateResponseLinesItem>) -> Self {
        self.lines = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1PayrollRunsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1PayrollRunsCreateResponseBuilder::id)
    /// - [`year`](PostV1PayrollRunsCreateResponseBuilder::year)
    /// - [`month`](PostV1PayrollRunsCreateResponseBuilder::month)
    /// - [`status`](PostV1PayrollRunsCreateResponseBuilder::status)
    /// - [`gross_total`](PostV1PayrollRunsCreateResponseBuilder::gross_total)
    /// - [`npd_total`](PostV1PayrollRunsCreateResponseBuilder::npd_total)
    /// - [`gpm_total`](PostV1PayrollRunsCreateResponseBuilder::gpm_total)
    /// - [`sodra_employee_total`](PostV1PayrollRunsCreateResponseBuilder::sodra_employee_total)
    /// - [`sodra_employer_total`](PostV1PayrollRunsCreateResponseBuilder::sodra_employer_total)
    /// - [`net_total`](PostV1PayrollRunsCreateResponseBuilder::net_total)
    /// - [`created_at`](PostV1PayrollRunsCreateResponseBuilder::created_at)
    /// - [`lines`](PostV1PayrollRunsCreateResponseBuilder::lines)
    pub fn build(self) -> Result<PostV1PayrollRunsCreateResponse, BuildError> {
        Ok(PostV1PayrollRunsCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            month: self
                .month
                .ok_or_else(|| BuildError::missing_field("month"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            gross_total: self
                .gross_total
                .ok_or_else(|| BuildError::missing_field("gross_total"))?,
            npd_total: self
                .npd_total
                .ok_or_else(|| BuildError::missing_field("npd_total"))?,
            gpm_total: self
                .gpm_total
                .ok_or_else(|| BuildError::missing_field("gpm_total"))?,
            sodra_employee_total: self
                .sodra_employee_total
                .ok_or_else(|| BuildError::missing_field("sodra_employee_total"))?,
            sodra_employer_total: self
                .sodra_employer_total
                .ok_or_else(|| BuildError::missing_field("sodra_employer_total"))?,
            net_total: self
                .net_total
                .ok_or_else(|| BuildError::missing_field("net_total"))?,
            journal_transaction_id: self.journal_transaction_id,
            notes: self.notes,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            approved_at: self.approved_at,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
        })
    }
}
