pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LtSdFfdataDeclarationsResponse {
    pub r#type: LtSdFfdataDeclarationsResponseType,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
    #[serde(default)]
    pub rows: i64,
    #[serde(rename = "pageCount")]
    #[serde(default)]
    pub page_count: i64,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl LtSdFfdataDeclarationsResponse {
    pub fn builder() -> LtSdFfdataDeclarationsResponseBuilder {
        <LtSdFfdataDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtSdFfdataDeclarationsResponseBuilder {
    r#type: Option<LtSdFfdataDeclarationsResponseType>,
    file_name: Option<String>,
    xml: Option<String>,
    rows: Option<i64>,
    page_count: Option<i64>,
    warnings: Option<Vec<String>>,
}

impl LtSdFfdataDeclarationsResponseBuilder {
    pub fn r#type(mut self, value: LtSdFfdataDeclarationsResponseType) -> Self {
        self.r#type = Some(value);
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

    pub fn rows(mut self, value: i64) -> Self {
        self.rows = Some(value);
        self
    }

    pub fn page_count(mut self, value: i64) -> Self {
        self.page_count = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LtSdFfdataDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](LtSdFfdataDeclarationsResponseBuilder::r#type)
    /// - [`file_name`](LtSdFfdataDeclarationsResponseBuilder::file_name)
    /// - [`xml`](LtSdFfdataDeclarationsResponseBuilder::xml)
    /// - [`rows`](LtSdFfdataDeclarationsResponseBuilder::rows)
    /// - [`page_count`](LtSdFfdataDeclarationsResponseBuilder::page_count)
    /// - [`warnings`](LtSdFfdataDeclarationsResponseBuilder::warnings)
    pub fn build(self) -> Result<LtSdFfdataDeclarationsResponse, BuildError> {
        Ok(LtSdFfdataDeclarationsResponse {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            page_count: self
                .page_count
                .ok_or_else(|| BuildError::missing_field("page_count"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
