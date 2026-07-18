pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsEuVatReturnComputeResponse {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "formKey")]
    #[serde(default)]
    pub form_key: String,
    #[serde(rename = "formName")]
    #[serde(default)]
    pub form_name: String,
    pub frequency: PostV1DeclarationsEuVatReturnComputeResponseFrequency,
    #[serde(rename = "periodStart")]
    #[serde(default)]
    pub period_start: String,
    #[serde(rename = "periodEnd")]
    #[serde(default)]
    pub period_end: String,
    #[serde(default)]
    pub boxes: Vec<PostV1DeclarationsEuVatReturnComputeResponseBoxesItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsEuVatReturnComputeResponse {
    pub fn builder() -> PostV1DeclarationsEuVatReturnComputeResponseBuilder {
        <PostV1DeclarationsEuVatReturnComputeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsEuVatReturnComputeResponseBuilder {
    country_code: Option<String>,
    form_key: Option<String>,
    form_name: Option<String>,
    frequency: Option<PostV1DeclarationsEuVatReturnComputeResponseFrequency>,
    period_start: Option<String>,
    period_end: Option<String>,
    boxes: Option<Vec<PostV1DeclarationsEuVatReturnComputeResponseBoxesItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PostV1DeclarationsEuVatReturnComputeResponseBuilder {
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

    pub fn frequency(
        mut self,
        value: PostV1DeclarationsEuVatReturnComputeResponseFrequency,
    ) -> Self {
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

    pub fn boxes(
        mut self,
        value: Vec<PostV1DeclarationsEuVatReturnComputeResponseBoxesItem>,
    ) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsEuVatReturnComputeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](PostV1DeclarationsEuVatReturnComputeResponseBuilder::country_code)
    /// - [`form_key`](PostV1DeclarationsEuVatReturnComputeResponseBuilder::form_key)
    /// - [`form_name`](PostV1DeclarationsEuVatReturnComputeResponseBuilder::form_name)
    /// - [`frequency`](PostV1DeclarationsEuVatReturnComputeResponseBuilder::frequency)
    /// - [`period_start`](PostV1DeclarationsEuVatReturnComputeResponseBuilder::period_start)
    /// - [`period_end`](PostV1DeclarationsEuVatReturnComputeResponseBuilder::period_end)
    /// - [`boxes`](PostV1DeclarationsEuVatReturnComputeResponseBuilder::boxes)
    /// - [`warnings`](PostV1DeclarationsEuVatReturnComputeResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsEuVatReturnComputeResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsEuVatReturnComputeResponseBuilder::source)
    pub fn build(self) -> Result<PostV1DeclarationsEuVatReturnComputeResponse, BuildError> {
        Ok(PostV1DeclarationsEuVatReturnComputeResponse {
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
