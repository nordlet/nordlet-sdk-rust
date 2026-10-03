pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsCyHe32GenerateResponse {
    #[serde(default)]
    pub year: i64,
    #[serde(rename = "madeUpTo")]
    #[serde(default)]
    pub made_up_to: String,
    #[serde(rename = "registrarNumber")]
    #[serde(default)]
    pub registrar_number: String,
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
    pub fields: Vec<PostV1DeclarationsCyHe32GenerateResponseFieldsItem>,
    #[serde(default)]
    pub members: Vec<PostV1DeclarationsCyHe32GenerateResponseMembersItem>,
    #[serde(default)]
    pub officers: Vec<PostV1DeclarationsCyHe32GenerateResponseOfficersItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl PostV1DeclarationsCyHe32GenerateResponse {
    pub fn builder() -> PostV1DeclarationsCyHe32GenerateResponseBuilder {
        <PostV1DeclarationsCyHe32GenerateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsCyHe32GenerateResponseBuilder {
    year: Option<i64>,
    made_up_to: Option<String>,
    registrar_number: Option<String>,
    company_name: Option<String>,
    file_name: Option<String>,
    xml: Option<String>,
    pdf_file_name: Option<String>,
    pdf: Option<String>,
    form_source: Option<String>,
    fields: Option<Vec<PostV1DeclarationsCyHe32GenerateResponseFieldsItem>>,
    members: Option<Vec<PostV1DeclarationsCyHe32GenerateResponseMembersItem>>,
    officers: Option<Vec<PostV1DeclarationsCyHe32GenerateResponseOfficersItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl PostV1DeclarationsCyHe32GenerateResponseBuilder {
    pub fn year(mut self, value: i64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn made_up_to(mut self, value: impl Into<String>) -> Self {
        self.made_up_to = Some(value.into());
        self
    }

    pub fn registrar_number(mut self, value: impl Into<String>) -> Self {
        self.registrar_number = Some(value.into());
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
        value: Vec<PostV1DeclarationsCyHe32GenerateResponseFieldsItem>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn members(
        mut self,
        value: Vec<PostV1DeclarationsCyHe32GenerateResponseMembersItem>,
    ) -> Self {
        self.members = Some(value);
        self
    }

    pub fn officers(
        mut self,
        value: Vec<PostV1DeclarationsCyHe32GenerateResponseOfficersItem>,
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

    /// Consumes the builder and constructs a [`PostV1DeclarationsCyHe32GenerateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](PostV1DeclarationsCyHe32GenerateResponseBuilder::year)
    /// - [`made_up_to`](PostV1DeclarationsCyHe32GenerateResponseBuilder::made_up_to)
    /// - [`registrar_number`](PostV1DeclarationsCyHe32GenerateResponseBuilder::registrar_number)
    /// - [`company_name`](PostV1DeclarationsCyHe32GenerateResponseBuilder::company_name)
    /// - [`file_name`](PostV1DeclarationsCyHe32GenerateResponseBuilder::file_name)
    /// - [`xml`](PostV1DeclarationsCyHe32GenerateResponseBuilder::xml)
    /// - [`pdf_file_name`](PostV1DeclarationsCyHe32GenerateResponseBuilder::pdf_file_name)
    /// - [`pdf`](PostV1DeclarationsCyHe32GenerateResponseBuilder::pdf)
    /// - [`form_source`](PostV1DeclarationsCyHe32GenerateResponseBuilder::form_source)
    /// - [`fields`](PostV1DeclarationsCyHe32GenerateResponseBuilder::fields)
    /// - [`members`](PostV1DeclarationsCyHe32GenerateResponseBuilder::members)
    /// - [`officers`](PostV1DeclarationsCyHe32GenerateResponseBuilder::officers)
    /// - [`warnings`](PostV1DeclarationsCyHe32GenerateResponseBuilder::warnings)
    /// - [`notes`](PostV1DeclarationsCyHe32GenerateResponseBuilder::notes)
    /// - [`source`](PostV1DeclarationsCyHe32GenerateResponseBuilder::source)
    pub fn build(self) -> Result<PostV1DeclarationsCyHe32GenerateResponse, BuildError> {
        Ok(PostV1DeclarationsCyHe32GenerateResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            made_up_to: self
                .made_up_to
                .ok_or_else(|| BuildError::missing_field("made_up_to"))?,
            registrar_number: self
                .registrar_number
                .ok_or_else(|| BuildError::missing_field("registrar_number"))?,
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
