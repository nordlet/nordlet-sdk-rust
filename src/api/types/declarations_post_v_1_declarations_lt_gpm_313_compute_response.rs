pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtGpm313ComputeResponse {
    #[serde(rename = "declarationYear")]
    #[serde(default)]
    pub declaration_year: i64,
    #[serde(rename = "declarationMonth")]
    #[serde(default)]
    pub declaration_month: i64,
    #[serde(rename = "runPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_period: Option<PostV1DeclarationsLtGpm313ComputeResponseRunPeriod>,
    #[serde(default)]
    pub fields: Vec<PostV1DeclarationsLtGpm313ComputeResponseFieldsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl PostV1DeclarationsLtGpm313ComputeResponse {
    pub fn builder() -> PostV1DeclarationsLtGpm313ComputeResponseBuilder {
        <PostV1DeclarationsLtGpm313ComputeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtGpm313ComputeResponseBuilder {
    declaration_year: Option<i64>,
    declaration_month: Option<i64>,
    run_period: Option<PostV1DeclarationsLtGpm313ComputeResponseRunPeriod>,
    fields: Option<Vec<PostV1DeclarationsLtGpm313ComputeResponseFieldsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl PostV1DeclarationsLtGpm313ComputeResponseBuilder {
    pub fn declaration_year(mut self, value: i64) -> Self {
        self.declaration_year = Some(value);
        self
    }

    pub fn declaration_month(mut self, value: i64) -> Self {
        self.declaration_month = Some(value);
        self
    }

    pub fn run_period(mut self, value: PostV1DeclarationsLtGpm313ComputeResponseRunPeriod) -> Self {
        self.run_period = Some(value);
        self
    }

    pub fn fields(
        mut self,
        value: Vec<PostV1DeclarationsLtGpm313ComputeResponseFieldsItem>,
    ) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtGpm313ComputeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`declaration_year`](PostV1DeclarationsLtGpm313ComputeResponseBuilder::declaration_year)
    /// - [`declaration_month`](PostV1DeclarationsLtGpm313ComputeResponseBuilder::declaration_month)
    /// - [`fields`](PostV1DeclarationsLtGpm313ComputeResponseBuilder::fields)
    /// - [`warnings`](PostV1DeclarationsLtGpm313ComputeResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsLtGpm313ComputeResponseBuilder::notes)
    pub fn build(self) -> Result<PostV1DeclarationsLtGpm313ComputeResponse, BuildError> {
        Ok(PostV1DeclarationsLtGpm313ComputeResponse {
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
