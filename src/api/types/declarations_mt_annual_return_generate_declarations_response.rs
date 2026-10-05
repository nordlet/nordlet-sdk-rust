pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MtAnnualReturnGenerateDeclarationsResponse {
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
    pub fields: Vec<MtAnnualReturnGenerateDeclarationsResponseFieldsItem>,
    #[serde(default)]
    pub members: Vec<MtAnnualReturnGenerateDeclarationsResponseMembersItem>,
    #[serde(default)]
    pub officers: Vec<MtAnnualReturnGenerateDeclarationsResponseOfficersItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl MtAnnualReturnGenerateDeclarationsResponse {
    pub fn builder() -> MtAnnualReturnGenerateDeclarationsResponseBuilder {
        <MtAnnualReturnGenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MtAnnualReturnGenerateDeclarationsResponseBuilder {
    year: Option<i64>,
    made_up_to: Option<String>,
    mbr_number: Option<String>,
    company_name: Option<String>,
    file_name: Option<String>,
    xml: Option<String>,
    pdf_file_name: Option<String>,
    pdf: Option<String>,
    form_source: Option<String>,
    fields: Option<Vec<MtAnnualReturnGenerateDeclarationsResponseFieldsItem>>,
    members: Option<Vec<MtAnnualReturnGenerateDeclarationsResponseMembersItem>>,
    officers: Option<Vec<MtAnnualReturnGenerateDeclarationsResponseOfficersItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl MtAnnualReturnGenerateDeclarationsResponseBuilder {
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
        value: Vec<MtAnnualReturnGenerateDeclarationsResponseFieldsItem>,
    ) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn members(
        mut self,
        value: Vec<MtAnnualReturnGenerateDeclarationsResponseMembersItem>,
    ) -> Self {
        self.members = Some(value);
        self
    }

    pub fn officers(
        mut self,
        value: Vec<MtAnnualReturnGenerateDeclarationsResponseOfficersItem>,
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

    /// Consumes the builder and constructs a [`MtAnnualReturnGenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](MtAnnualReturnGenerateDeclarationsResponseBuilder::year)
    /// - [`mbr_number`](MtAnnualReturnGenerateDeclarationsResponseBuilder::mbr_number)
    /// - [`company_name`](MtAnnualReturnGenerateDeclarationsResponseBuilder::company_name)
    /// - [`file_name`](MtAnnualReturnGenerateDeclarationsResponseBuilder::file_name)
    /// - [`xml`](MtAnnualReturnGenerateDeclarationsResponseBuilder::xml)
    /// - [`pdf_file_name`](MtAnnualReturnGenerateDeclarationsResponseBuilder::pdf_file_name)
    /// - [`pdf`](MtAnnualReturnGenerateDeclarationsResponseBuilder::pdf)
    /// - [`form_source`](MtAnnualReturnGenerateDeclarationsResponseBuilder::form_source)
    /// - [`fields`](MtAnnualReturnGenerateDeclarationsResponseBuilder::fields)
    /// - [`members`](MtAnnualReturnGenerateDeclarationsResponseBuilder::members)
    /// - [`officers`](MtAnnualReturnGenerateDeclarationsResponseBuilder::officers)
    /// - [`warnings`](MtAnnualReturnGenerateDeclarationsResponseBuilder::warnings)
    /// - [`notes`](MtAnnualReturnGenerateDeclarationsResponseBuilder::notes)
    /// - [`source`](MtAnnualReturnGenerateDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<MtAnnualReturnGenerateDeclarationsResponse, BuildError> {
        Ok(MtAnnualReturnGenerateDeclarationsResponse {
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
