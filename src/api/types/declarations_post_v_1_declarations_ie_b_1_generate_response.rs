pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsIeB1GenerateResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "croNumber")]
    #[serde(default)]
    pub cro_number: String,
    #[serde(rename = "companyName")]
    #[serde(default)]
    pub company_name: String,
    #[serde(rename = "annualReturnDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_return_date: Option<String>,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
    #[serde(default)]
    pub fields: Vec<PostV1DeclarationsIeB1GenerateResponseFieldsItem>,
    #[serde(default)]
    pub directors: Vec<PostV1DeclarationsIeB1GenerateResponseDirectorsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secretary: Option<PostV1DeclarationsIeB1GenerateResponseSecretary>,
    #[serde(default)]
    pub members: Vec<PostV1DeclarationsIeB1GenerateResponseMembersItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsIeB1GenerateResponse {
    pub fn builder() -> PostV1DeclarationsIeB1GenerateResponseBuilder {
        <PostV1DeclarationsIeB1GenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsIeB1GenerateResponseBuilder {
    year: Option<i64>,
    cro_number: Option<String>,
    company_name: Option<String>,
    annual_return_date: Option<String>,
    file_name: Option<String>,
    xml: Option<String>,
    fields: Option<Vec<PostV1DeclarationsIeB1GenerateResponseFieldsItem>>,
    directors: Option<Vec<PostV1DeclarationsIeB1GenerateResponseDirectorsItem>>,
    secretary: Option<PostV1DeclarationsIeB1GenerateResponseSecretary>,
    members: Option<Vec<PostV1DeclarationsIeB1GenerateResponseMembersItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PostV1DeclarationsIeB1GenerateResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn cro_number(mut self, value: impl Into<String>) -> Self {
        self.cro_number = Some(value.into());
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
        self
    }

    pub fn annual_return_date(mut self, value: impl Into<String>) -> Self {
        self.annual_return_date = Some(value.into());
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

    pub fn fields(mut self, value: Vec<PostV1DeclarationsIeB1GenerateResponseFieldsItem>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn directors(
        mut self,
        value: Vec<PostV1DeclarationsIeB1GenerateResponseDirectorsItem>,
    ) -> Self {
        self.directors = Some(value);
        self
    }

    pub fn secretary(mut self, value: PostV1DeclarationsIeB1GenerateResponseSecretary) -> Self {
        self.secretary = Some(value);
        self
    }

    pub fn members(
        mut self,
        value: Vec<PostV1DeclarationsIeB1GenerateResponseMembersItem>,
    ) -> Self {
        self.members = Some(value);
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsIeB1GenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsIeB1GenerateResponseBuilder::year)
    /// - [`cro_number`](PostV1DeclarationsIeB1GenerateResponseBuilder::cro_number)
    /// - [`company_name`](PostV1DeclarationsIeB1GenerateResponseBuilder::company_name)
    /// - [`file_name`](PostV1DeclarationsIeB1GenerateResponseBuilder::file_name)
    /// - [`xml`](PostV1DeclarationsIeB1GenerateResponseBuilder::xml)
    /// - [`fields`](PostV1DeclarationsIeB1GenerateResponseBuilder::fields)
    /// - [`directors`](PostV1DeclarationsIeB1GenerateResponseBuilder::directors)
    /// - [`members`](PostV1DeclarationsIeB1GenerateResponseBuilder::members)
    /// - [`warnings`](PostV1DeclarationsIeB1GenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsIeB1GenerateResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsIeB1GenerateResponseBuilder::source)
    pub fn build(self) -> Result<PostV1DeclarationsIeB1GenerateResponse, BuildError> {
        Ok(PostV1DeclarationsIeB1GenerateResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            cro_number: self
                .cro_number
                .ok_or_else(|| BuildError::missing_field("cro_number"))?,
            company_name: self
                .company_name
                .ok_or_else(|| BuildError::missing_field("company_name"))?,
            annual_return_date: self.annual_return_date,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
            directors: self
                .directors
                .ok_or_else(|| BuildError::missing_field("directors"))?,
            secretary: self.secretary,
            members: self
                .members
                .ok_or_else(|| BuildError::missing_field("members"))?,
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
