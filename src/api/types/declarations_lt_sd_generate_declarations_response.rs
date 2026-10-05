pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LtSdGenerateDeclarationsResponse {
    pub r#type: LtSdGenerateDeclarationsResponseType,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: NaiveDate,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: NaiveDate,
    #[serde(default)]
    pub rows: Vec<LtSdGenerateDeclarationsResponseRowsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl LtSdGenerateDeclarationsResponse {
    pub fn builder() -> LtSdGenerateDeclarationsResponseBuilder {
        <LtSdGenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LtSdGenerateDeclarationsResponseBuilder {
    r#type: Option<LtSdGenerateDeclarationsResponseType>,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
    rows: Option<Vec<LtSdGenerateDeclarationsResponseRowsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl LtSdGenerateDeclarationsResponseBuilder {
    pub fn r#type(mut self, value: LtSdGenerateDeclarationsResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn from_date(mut self, value: NaiveDate) -> Self {
        self.from_date = Some(value);
        self
    }

    pub fn to_date(mut self, value: NaiveDate) -> Self {
        self.to_date = Some(value);
        self
    }

    pub fn rows(mut self, value: Vec<LtSdGenerateDeclarationsResponseRowsItem>) -> Self {
        self.rows = Some(value);
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

    /// Consumes the builder and constructs a [`LtSdGenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](LtSdGenerateDeclarationsResponseBuilder::r#type)
    /// - [`from_date`](LtSdGenerateDeclarationsResponseBuilder::from_date)
    /// - [`to_date`](LtSdGenerateDeclarationsResponseBuilder::to_date)
    /// - [`rows`](LtSdGenerateDeclarationsResponseBuilder::rows)
    /// - [`warnings`](LtSdGenerateDeclarationsResponseBuilder::warnings)
    /// - [`notes`](LtSdGenerateDeclarationsResponseBuilder::notes)
    pub fn build(self) -> Result<LtSdGenerateDeclarationsResponse, BuildError> {
        Ok(LtSdGenerateDeclarationsResponse {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            from_date: self
                .from_date
                .ok_or_else(|| BuildError::missing_field("from_date"))?,
            to_date: self
                .to_date
                .ok_or_else(|| BuildError::missing_field("to_date"))?,
            rows: self.rows.ok_or_else(|| BuildError::missing_field("rows"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            notes: self
                .notes
                .ok_or_else(|| BuildError::missing_field("notes"))?,
        })
    }
}
