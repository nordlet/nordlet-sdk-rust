pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtGpm313ComputeDeclarationsResponse {
    #[serde(rename = "declarationYear")]
    #[serde(default)]
    pub declaration_year: i64,
    #[serde(rename = "declarationMonth")]
    #[serde(default)]
    pub declaration_month: i64,
    #[serde(rename = "runPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_period: Option<LtGpm313ComputeDeclarationsResponseRunPeriod>,
    #[serde(default)]
    pub fields: Vec<LtGpm313ComputeDeclarationsResponseFieldsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl LtGpm313ComputeDeclarationsResponse {
    pub fn builder() -> LtGpm313ComputeDeclarationsResponseBuilder {
        <LtGpm313ComputeDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtGpm313ComputeDeclarationsResponseBuilder {
    declaration_year: Option<i64>,
    declaration_month: Option<i64>,
    run_period: Option<LtGpm313ComputeDeclarationsResponseRunPeriod>,
    fields: Option<Vec<LtGpm313ComputeDeclarationsResponseFieldsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl LtGpm313ComputeDeclarationsResponseBuilder {
    pub fn declaration_year(mut self, value: i64) -> Self {
        self.declaration_year = Some(value);
        self
    }

    pub fn declaration_month(mut self, value: i64) -> Self {
        self.declaration_month = Some(value);
        self
    }

    pub fn run_period(mut self, value: LtGpm313ComputeDeclarationsResponseRunPeriod) -> Self {
        self.run_period = Some(value);
        self
    }

    pub fn fields(mut self, value: Vec<LtGpm313ComputeDeclarationsResponseFieldsItem>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtGpm313ComputeDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`declaration_year`](LtGpm313ComputeDeclarationsResponseBuilder::declaration_year)
    /// - [`declaration_month`](LtGpm313ComputeDeclarationsResponseBuilder::declaration_month)
    /// - [`fields`](LtGpm313ComputeDeclarationsResponseBuilder::fields)
    /// - [`warnings`](LtGpm313ComputeDeclarationsResponseBuilder::warnings)
    /// - [`notes`](LtGpm313ComputeDeclarationsResponseBuilder::notes)
    pub fn build(self) -> Result<LtGpm313ComputeDeclarationsResponse, BuildError> {
        Ok(LtGpm313ComputeDeclarationsResponse {
            declaration_year: self
                .declaration_year
                .ok_or_else(|| BuildError::missing_field("declaration_year"))?,
            declaration_month: self
                .declaration_month
                .ok_or_else(|| BuildError::missing_field("declaration_month"))?,
            run_period: self.run_period,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
        })
    }
}
