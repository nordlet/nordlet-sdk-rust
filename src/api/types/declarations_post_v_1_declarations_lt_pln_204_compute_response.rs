pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostV1DeclarationsLtPln204ComputeResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    pub variant: PostV1DeclarationsLtPln204ComputeResponseVariant,
    #[serde(rename = "registrationNumber")]
    #[serde(default)]
    pub registration_number: String,
    #[serde(rename = "companyName")]
    #[serde(default)]
    pub company_name: String,
    #[serde(rename = "ratePercent")]
    #[serde(default)]
    pub rate_percent: String,
    #[serde(rename = "rateCode")]
    #[serde(default)]
    pub rate_code: String,
    #[serde(rename = "smallEntity")]
    #[serde(default)]
    pub small_entity: bool,
    #[serde(default)]
    pub criteria: PostV1DeclarationsLtPln204ComputeResponseCriteria,
    #[serde(rename = "totalIncome")]
    #[serde(default)]
    pub total_income: String,
    #[serde(default)]
    pub boxes: HashMap<String, String>,
    #[serde(rename = "annexS")]
    #[serde(default)]
    pub annex_s: Vec<PostV1DeclarationsLtPln204ComputeResponseAnnexSItem>,
    #[serde(rename = "annexZ")]
    #[serde(default)]
    pub annex_z: Vec<PostV1DeclarationsLtPln204ComputeResponseAnnexZItem>,
    #[serde(default)]
    pub lines: Vec<PostV1DeclarationsLtPln204ComputeResponseLinesItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsLtPln204ComputeResponse {
    pub fn builder() -> PostV1DeclarationsLtPln204ComputeResponseBuilder {
        <PostV1DeclarationsLtPln204ComputeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtPln204ComputeResponseBuilder {
    year: Option<i64>,
    period_start: Option<String>,
    period_end: Option<String>,
    variant: Option<PostV1DeclarationsLtPln204ComputeResponseVariant>,
    registration_number: Option<String>,
    company_name: Option<String>,
    rate_percent: Option<String>,
    rate_code: Option<String>,
    small_entity: Option<bool>,
    criteria: Option<PostV1DeclarationsLtPln204ComputeResponseCriteria>,
    total_income: Option<String>,
    boxes: Option<HashMap<String, String>>,
    annex_s: Option<Vec<PostV1DeclarationsLtPln204ComputeResponseAnnexSItem>>,
    annex_z: Option<Vec<PostV1DeclarationsLtPln204ComputeResponseAnnexZItem>>,
    lines: Option<Vec<PostV1DeclarationsLtPln204ComputeResponseLinesItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PostV1DeclarationsLtPln204ComputeResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn period_start(mut self, value: impl Into<String>) -> Self {
        self.period_start = Some(value.into());
        self
    }

    pub fn period_end(mut self, value: impl Into<String>) -> Self {
        self.period_end = Some(value.into());
        self
    }

    pub fn variant(mut self, value: PostV1DeclarationsLtPln204ComputeResponseVariant) -> Self {
        self.variant = Some(value);
        self
    }

    pub fn registration_number(mut self, value: impl Into<String>) -> Self {
        self.registration_number = Some(value.into());
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn rate_percent(mut self, value: impl Into<String>) -> Self {
        self.rate_percent = Some(value.into());
        self
    }

    pub fn rate_code(mut self, value: impl Into<String>) -> Self {
        self.rate_code = Some(value.into());
        self
    }

    pub fn small_entity(mut self, value: bool) -> Self {
        self.small_entity = Some(value);
        self
    }

    pub fn criteria(mut self, value: PostV1DeclarationsLtPln204ComputeResponseCriteria) -> Self {
        self.criteria = Some(value);
        self
    }

    pub fn total_income(mut self, value: impl Into<String>) -> Self {
        self.total_income = Some(value.into());
        self
    }

    pub fn boxes(mut self, value: HashMap<String, String>) -> Self {
        self.boxes = Some(value);
        self
    }

    pub fn annex_s(
        mut self,
        value: Vec<PostV1DeclarationsLtPln204ComputeResponseAnnexSItem>,
    ) -> Self {
        self.annex_s = Some(value);
        self
    }

    pub fn annex_z(
        mut self,
        value: Vec<PostV1DeclarationsLtPln204ComputeResponseAnnexZItem>,
    ) -> Self {
        self.annex_z = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<PostV1DeclarationsLtPln204ComputeResponseLinesItem>) -> Self {
        self.lines = Some(value);
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

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtPln204ComputeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsLtPln204ComputeResponseBuilder::year)
    /// - [`period_start`](PostV1DeclarationsLtPln204ComputeResponseBuilder::period_start)
    /// - [`period_end`](PostV1DeclarationsLtPln204ComputeResponseBuilder::period_end)
    /// - [`variant`](PostV1DeclarationsLtPln204ComputeResponseBuilder::variant)
    /// - [`registration_number`](PostV1DeclarationsLtPln204ComputeResponseBuilder::registration_number)
    /// - [`company_name`](PostV1DeclarationsLtPln204ComputeResponseBuilder::company_name)
    /// - [`rate_percent`](PostV1DeclarationsLtPln204ComputeResponseBuilder::rate_percent)
    /// - [`rate_code`](PostV1DeclarationsLtPln204ComputeResponseBuilder::rate_code)
    /// - [`small_entity`](PostV1DeclarationsLtPln204ComputeResponseBuilder::small_entity)
    /// - [`criteria`](PostV1DeclarationsLtPln204ComputeResponseBuilder::criteria)
    /// - [`total_income`](PostV1DeclarationsLtPln204ComputeResponseBuilder::total_income)
    /// - [`boxes`](PostV1DeclarationsLtPln204ComputeResponseBuilder::boxes)
    /// - [`annex_s`](PostV1DeclarationsLtPln204ComputeResponseBuilder::annex_s)
    /// - [`annex_z`](PostV1DeclarationsLtPln204ComputeResponseBuilder::annex_z)
    /// - [`lines`](PostV1DeclarationsLtPln204ComputeResponseBuilder::lines)
    /// - [`warnings`](PostV1DeclarationsLtPln204ComputeResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsLtPln204ComputeResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsLtPln204ComputeResponseBuilder::source)
    pub fn build(self) -> Result<PostV1DeclarationsLtPln204ComputeResponse, BuildError> {
        Ok(PostV1DeclarationsLtPln204ComputeResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            variant: self
                .variant
                .ok_or_else(|| BuildError::missing_field("variant"))?,
            registration_number: self
                .registration_number
                .ok_or_else(|| BuildError::missing_field("registration_number"))?,
            company_name: self
                .company_name
                .ok_or_else(|| BuildError::missing_field("company_name"))?,
            rate_percent: self
                .rate_percent
                .ok_or_else(|| BuildError::missing_field("rate_percent"))?,
            rate_code: self
                .rate_code
                .ok_or_else(|| BuildError::missing_field("rate_code"))?,
            small_entity: self
                .small_entity
                .ok_or_else(|| BuildError::missing_field("small_entity"))?,
            criteria: self
                .criteria
                .ok_or_else(|| BuildError::missing_field("criteria"))?,
            total_income: self
                .total_income
                .ok_or_else(|| BuildError::missing_field("total_income"))?,
            boxes: self
                .boxes
                .ok_or_else(|| BuildError::missing_field("boxes"))?,
            annex_s: self
                .annex_s
                .ok_or_else(|| BuildError::missing_field("annex_s"))?,
            annex_z: self
                .annex_z
                .ok_or_else(|| BuildError::missing_field("annex_z"))?,
            lines: self
                .lines
                .ok_or_else(|| BuildError::missing_field("lines"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
