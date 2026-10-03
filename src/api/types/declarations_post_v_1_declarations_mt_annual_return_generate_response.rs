pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsMtAnnualReturnGenerateResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "madeUpTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub made_up_to: Option<String>,
    #[serde(rename = "mbrNumber")]
    #[serde(default)]
    pub mbr_number: String,
    #[serde(rename = "companyName")]
    #[serde(default)]
    pub company_name: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
    #[serde(rename = "pdfFileName")]
    #[serde(default)]
    pub pdf_file_name: String,
    #[serde(default)]
    pub pdf: String,
    #[serde(rename = "formSource")]
    #[serde(default)]
    pub form_source: String,
    #[serde(default)]
    pub fields: Vec<PostV1DeclarationsMtAnnualReturnGenerateResponseFieldsItem>,
    #[serde(default)]
    pub members: Vec<PostV1DeclarationsMtAnnualReturnGenerateResponseMembersItem>,
    #[serde(default)]
    pub officers: Vec<PostV1DeclarationsMtAnnualReturnGenerateResponseOfficersItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsMtAnnualReturnGenerateResponse {
    pub fn builder() -> PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder {
        <PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder {
    year: Option<i64>,
    made_up_to: Option<String>,
    mbr_number: Option<String>,
    company_name: Option<String>,
    file_name: Option<String>,
    xml: Option<String>,
    pdf_file_name: Option<String>,
    pdf: Option<String>,
    form_source: Option<String>,
    fields: Option<Vec<PostV1DeclarationsMtAnnualReturnGenerateResponseFieldsItem>>,
    members: Option<Vec<PostV1DeclarationsMtAnnualReturnGenerateResponseMembersItem>>,
    officers: Option<Vec<PostV1DeclarationsMtAnnualReturnGenerateResponseOfficersItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn made_up_to(mut self, value: impl Into<String>) -> Self {
        self.made_up_to = Some(value.into());
        self
    }

    pub fn mbr_number(mut self, value: impl Into<String>) -> Self {
        self.mbr_number = Some(value.into());
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
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

    pub fn pdf_file_name(mut self, value: impl Into<String>) -> Self {
        self.pdf_file_name = Some(value.into());
        self
    }

    pub fn pdf(mut self, value: impl Into<String>) -> Self {
        self.pdf = Some(value.into());
        self
    }

    pub fn form_source(mut self, value: impl Into<String>) -> Self {
        self.form_source = Some(value.into());
        self
    }

    pub fn fields(
        mut self,
        value: Vec<PostV1DeclarationsMtAnnualReturnGenerateResponseFieldsItem>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn members(
        mut self,
        value: Vec<PostV1DeclarationsMtAnnualReturnGenerateResponseMembersItem>,
    ) -> Self {
        self.members = Some(value);
        self
    }

    pub fn officers(
        mut self,
        value: Vec<PostV1DeclarationsMtAnnualReturnGenerateResponseOfficersItem>,
    ) -> Self {
        self.officers = Some(value);
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsMtAnnualReturnGenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder::year)
    /// - [`mbr_number`](PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder::mbr_number)
    /// - [`company_name`](PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder::company_name)
    /// - [`file_name`](PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder::file_name)
    /// - [`xml`](PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder::xml)
    /// - [`pdf_file_name`](PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder::pdf_file_name)
    /// - [`pdf`](PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder::pdf)
    /// - [`form_source`](PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder::form_source)
    /// - [`fields`](PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder::fields)
    /// - [`members`](PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder::members)
    /// - [`officers`](PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder::officers)
    /// - [`warnings`](PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsMtAnnualReturnGenerateResponseBuilder::source)
    pub fn build(self) -> Result<PostV1DeclarationsMtAnnualReturnGenerateResponse, BuildError> {
        Ok(PostV1DeclarationsMtAnnualReturnGenerateResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            made_up_to: self.made_up_to,
            mbr_number: self
                .mbr_number
                .ok_or_else(|| BuildError::missing_field("mbr_number"))?,
            company_name: self
                .company_name
                .ok_or_else(|| BuildError::missing_field("company_name"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
            pdf_file_name: self
                .pdf_file_name
                .ok_or_else(|| BuildError::missing_field("pdf_file_name"))?,
            pdf: self.pdf.ok_or_else(|| BuildError::missing_field("pdf"))?,
            form_source: self
                .form_source
                .ok_or_else(|| BuildError::missing_field("form_source"))?,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
            members: self
                .members
                .ok_or_else(|| BuildError::missing_field("members"))?,
            officers: self
                .officers
                .ok_or_else(|| BuildError::missing_field("officers"))?,
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
