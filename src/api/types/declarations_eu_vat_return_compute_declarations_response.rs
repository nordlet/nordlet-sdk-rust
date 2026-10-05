pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EuVatReturnComputeDeclarationsResponse {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "formKey")]
    #[serde(default)]
    pub form_key: String,
    #[serde(rename = "formName")]
    #[serde(default)]
    pub form_name: String,
    pub frequency: EuVatReturnComputeDeclarationsResponseFrequency,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(default)]
    pub boxes: Vec<EuVatReturnComputeDeclarationsResponseBoxesItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl EuVatReturnComputeDeclarationsResponse {
    pub fn builder() -> EuVatReturnComputeDeclarationsResponseBuilder {
        <EuVatReturnComputeDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuVatReturnComputeDeclarationsResponseBuilder {
    country_code: Option<String>,
    form_key: Option<String>,
    form_name: Option<String>,
    frequency: Option<EuVatReturnComputeDeclarationsResponseFrequency>,
    period_start: Option<String>,
    period_end: Option<String>,
    boxes: Option<Vec<EuVatReturnComputeDeclarationsResponseBoxesItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl EuVatReturnComputeDeclarationsResponseBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn form_key(mut self, value: impl Into<String>) -> Self {
        self.form_key = Some(value.into());
        self
    }

    pub fn form_name(mut self, value: impl Into<String>) -> Self {
        self.form_name = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: EuVatReturnComputeDeclarationsResponseFrequency) -> Self {
        self.frequency = Some(value);
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

    pub fn boxes(mut self, value: Vec<EuVatReturnComputeDeclarationsResponseBoxesItem>) -> Self {
        self.boxes = Some(value);
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

    /// Consumes the builder and constructs a [`EuVatReturnComputeDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](EuVatReturnComputeDeclarationsResponseBuilder::country_code)
    /// - [`form_key`](EuVatReturnComputeDeclarationsResponseBuilder::form_key)
    /// - [`form_name`](EuVatReturnComputeDeclarationsResponseBuilder::form_name)
    /// - [`frequency`](EuVatReturnComputeDeclarationsResponseBuilder::frequency)
    /// - [`period_start`](EuVatReturnComputeDeclarationsResponseBuilder::period_start)
    /// - [`period_end`](EuVatReturnComputeDeclarationsResponseBuilder::period_end)
    /// - [`boxes`](EuVatReturnComputeDeclarationsResponseBuilder::boxes)
    /// - [`warnings`](EuVatReturnComputeDeclarationsResponseBuilder::warnings)
    /// - [`notes`](EuVatReturnComputeDeclarationsResponseBuilder::notes)
    /// - [`source`](EuVatReturnComputeDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<EuVatReturnComputeDeclarationsResponse, BuildError> {
        Ok(EuVatReturnComputeDeclarationsResponse {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            form_key: self
                .form_key
                .ok_or_else(|| BuildError::missing_field("form_key"))?,
            form_name: self
                .form_name
                .ok_or_else(|| BuildError::missing_field("form_name"))?,
            frequency: self
                .frequency
                .ok_or_else(|| BuildError::missing_field("frequency"))?,
            period_start: self
                .period_start
                .ok_or_else(|| BuildError::missing_field("period_start"))?,
            period_end: self
                .period_end
                .ok_or_else(|| BuildError::missing_field("period_end"))?,
            boxes: self
                .boxes
                .ok_or_else(|| BuildError::missing_field("boxes"))?,
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
