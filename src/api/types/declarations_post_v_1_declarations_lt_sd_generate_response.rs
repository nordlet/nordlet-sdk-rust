pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLtSdGenerateResponse {
    pub r#type: PostV1DeclarationsLtSdGenerateResponseType,
    #[serde(rename = "fromDate")]
    #[serde(default)]
    pub from_date: String,
    #[serde(rename = "toDate")]
    #[serde(default)]
    pub to_date: String,
    #[serde(default)]
    pub rows: Vec<PostV1DeclarationsLtSdGenerateResponseRowsItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl PostV1DeclarationsLtSdGenerateResponse {
    pub fn builder() -> PostV1DeclarationsLtSdGenerateResponseBuilder {
        <PostV1DeclarationsLtSdGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLtSdGenerateResponseBuilder {
    r#type: Option<PostV1DeclarationsLtSdGenerateResponseType>,
    from_date: Option<String>,
    to_date: Option<String>,
    rows: Option<Vec<PostV1DeclarationsLtSdGenerateResponseRowsItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
}

impl PostV1DeclarationsLtSdGenerateResponseBuilder {
    pub fn r#type(mut self, value: PostV1DeclarationsLtSdGenerateResponseType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn from_date(mut self, value: impl Into<String>) -> Self {
        self.from_date = Some(value.into());
        self
    }

    pub fn to_date(mut self, value: impl Into<String>) -> Self {
        self.to_date = Some(value.into());
        self
    }

    pub fn rows(mut self, value: Vec<PostV1DeclarationsLtSdGenerateResponseRowsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsLtSdGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](PostV1DeclarationsLtSdGenerateResponseBuilder::r#type)
    /// - [`from_date`](PostV1DeclarationsLtSdGenerateResponseBuilder::from_date)
    /// - [`to_date`](PostV1DeclarationsLtSdGenerateResponseBuilder::to_date)
    /// - [`rows`](PostV1DeclarationsLtSdGenerateResponseBuilder::rows)
    /// - [`warnings`](PostV1DeclarationsLtSdGenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsLtSdGenerateResponseBuilder::notes)
    pub fn build(self) -> Result<PostV1DeclarationsLtSdGenerateResponse, BuildError> {
        Ok(PostV1DeclarationsLtSdGenerateResponse {
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
