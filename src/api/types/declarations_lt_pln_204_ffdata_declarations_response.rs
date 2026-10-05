pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LtPln204FfdataDeclarationsResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
    #[serde(rename = "ratePercent")]
    #[serde(default)]
    pub rate_percent: String,
    #[serde(rename = "rateCode")]
    #[serde(default)]
    pub rate_code: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl LtPln204FfdataDeclarationsResponse {
    pub fn builder() -> LtPln204FfdataDeclarationsResponseBuilder {
        <LtPln204FfdataDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtPln204FfdataDeclarationsResponseBuilder {
    year: Option<i64>,
    file_name: Option<String>,
    xml: Option<String>,
    rate_percent: Option<String>,
    rate_code: Option<String>,
    warnings: Option<Vec<String>>,
}

impl LtPln204FfdataDeclarationsResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn xml(mut self, value: impl Into<String>) -> Self {
        self.xml = Some(value.into());
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

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtPln204FfdataDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](LtPln204FfdataDeclarationsResponseBuilder::year)
    /// - [`file_name`](LtPln204FfdataDeclarationsResponseBuilder::file_name)
    /// - [`xml`](LtPln204FfdataDeclarationsResponseBuilder::xml)
    /// - [`rate_percent`](LtPln204FfdataDeclarationsResponseBuilder::rate_percent)
    /// - [`rate_code`](LtPln204FfdataDeclarationsResponseBuilder::rate_code)
    /// - [`warnings`](LtPln204FfdataDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<LtPln204FfdataDeclarationsResponse, BuildError> {
        Ok(LtPln204FfdataDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
            rate_percent: self
                .rate_percent
                .ok_or_else(|| BuildError::missing_field("rate_percent"))?,
            rate_code: self
                .rate_code
                .ok_or_else(|| BuildError::missing_field("rate_code"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
