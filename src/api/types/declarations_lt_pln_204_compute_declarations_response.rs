pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LtPln204ComputeDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    pub variant: LtPln204ComputeDeclarationsResponseVariant,
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
    pub criteria: LtPln204ComputeDeclarationsResponseCriteria,
    #[serde(rename = "totalIncome")]
    #[serde(default)]
    pub total_income: String,
    #[serde(default)]
    pub boxes: HashMap<String, String>,
    #[serde(rename = "annexS")]
    #[serde(default)]
    pub annex_s: Vec<LtPln204ComputeDeclarationsResponseAnnexSItem>,
    #[serde(rename = "annexZ")]
    #[serde(default)]
    pub annex_z: Vec<LtPln204ComputeDeclarationsResponseAnnexZItem>,
    #[serde(default)]
    pub lines: Vec<LtPln204ComputeDeclarationsResponseLinesItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl LtPln204ComputeDeclarationsResponse {
    pub fn builder() -> LtPln204ComputeDeclarationsResponseBuilder {
        <LtPln204ComputeDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtPln204ComputeDeclarationsResponseBuilder {
    year: Option<i64>,
    period_start: Option<String>,
    period_end: Option<String>,
    variant: Option<LtPln204ComputeDeclarationsResponseVariant>,
    registration_number: Option<String>,
    company_name: Option<String>,
    rate_percent: Option<String>,
    rate_code: Option<String>,
    small_entity: Option<bool>,
    criteria: Option<LtPln204ComputeDeclarationsResponseCriteria>,
    total_income: Option<String>,
    boxes: Option<HashMap<String, String>>,
    annex_s: Option<Vec<LtPln204ComputeDeclarationsResponseAnnexSItem>>,
    annex_z: Option<Vec<LtPln204ComputeDeclarationsResponseAnnexZItem>>,
    lines: Option<Vec<LtPln204ComputeDeclarationsResponseLinesItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl LtPln204ComputeDeclarationsResponseBuilder {
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

    pub fn variant(mut self, value: LtPln204ComputeDeclarationsResponseVariant) -> Self {
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

    pub fn criteria(mut self, value: LtPln204ComputeDeclarationsResponseCriteria) -> Self {
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

    pub fn annex_s(mut self, value: Vec<LtPln204ComputeDeclarationsResponseAnnexSItem>) -> Self {
        self.annex_s = Some(value);
        self
    }

    pub fn annex_z(mut self, value: Vec<LtPln204ComputeDeclarationsResponseAnnexZItem>) -> Self {
        self.annex_z = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<LtPln204ComputeDeclarationsResponseLinesItem>) -> Self {
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

    /// Consumes the builder and constructs a [`LtPln204ComputeDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtPln204ComputeDeclarationsResponseBuilder::year)
    /// - [`period_start`](LtPln204ComputeDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](LtPln204ComputeDeclarationsResponseBuilder::period_end)
    /// - [`variant`](LtPln204ComputeDeclarationsResponseBuilder::variant)
    /// - [`registration_number`](LtPln204ComputeDeclarationsResponseBuilder::registration_number)
    /// - [`company_name`](LtPln204ComputeDeclarationsResponseBuilder::company_name)
    /// - [`rate_percent`](LtPln204ComputeDeclarationsResponseBuilder::rate_percent)
    /// - [`rate_code`](LtPln204ComputeDeclarationsResponseBuilder::rate_code)
    /// - [`small_entity`](LtPln204ComputeDeclarationsResponseBuilder::small_entity)
    /// - [`criteria`](LtPln204ComputeDeclarationsResponseBuilder::criteria)
    /// - [`total_income`](LtPln204ComputeDeclarationsResponseBuilder::total_income)
    /// - [`boxes`](LtPln204ComputeDeclarationsResponseBuilder::boxes)
    /// - [`annex_s`](LtPln204ComputeDeclarationsResponseBuilder::annex_s)
    /// - [`annex_z`](LtPln204ComputeDeclarationsResponseBuilder::annex_z)
    /// - [`lines`](LtPln204ComputeDeclarationsResponseBuilder::lines)
    /// - [`warnings`](LtPln204ComputeDeclarationsResponseBuilder::warnings)
    /// - [`notes`](LtPln204ComputeDeclarationsResponseBuilder::notes)
    /// - [`source`](LtPln204ComputeDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<LtPln204ComputeDeclarationsResponse, BuildError> {
        Ok(LtPln204ComputeDeclarationsResponse {
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
