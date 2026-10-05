pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IeB1GenerateDeclarationsResponse {
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
    pub annual_return_date: Option<NaiveDate>,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
    #[serde(default)]
    pub fields: Vec<IeB1GenerateDeclarationsResponseFieldsItem>,
    #[serde(default)]
    pub directors: Vec<IeB1GenerateDeclarationsResponseDirectorsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secretary: Option<IeB1GenerateDeclarationsResponseSecretary>,
    #[serde(default)]
    pub members: Vec<IeB1GenerateDeclarationsResponseMembersItem>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub source: String,
}

impl IeB1GenerateDeclarationsResponse {
    pub fn builder() -> IeB1GenerateDeclarationsResponseBuilder {
        <IeB1GenerateDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IeB1GenerateDeclarationsResponseBuilder {
    year: Option<i64>,
    cro_number: Option<String>,
    company_name: Option<String>,
    annual_return_date: Option<NaiveDate>,
    file_name: Option<String>,
    xml: Option<String>,
    fields: Option<Vec<IeB1GenerateDeclarationsResponseFieldsItem>>,
    directors: Option<Vec<IeB1GenerateDeclarationsResponseDirectorsItem>>,
    secretary: Option<IeB1GenerateDeclarationsResponseSecretary>,
    members: Option<Vec<IeB1GenerateDeclarationsResponseMembersItem>>,
    warnings: Option<Vec<String>>,
    notes: Option<Vec<String>>,
    source: Option<String>,
}

impl IeB1GenerateDeclarationsResponseBuilder {
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

    pub fn annual_return_date(mut self, value: NaiveDate) -> Self {
        self.annual_return_date = Some(value);
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

    pub fn fields(mut self, value: Vec<IeB1GenerateDeclarationsResponseFieldsItem>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn directors(mut self, value: Vec<IeB1GenerateDeclarationsResponseDirectorsItem>) -> Self {
        self.directors = Some(value);
        self
    }

    pub fn secretary(mut self, value: IeB1GenerateDeclarationsResponseSecretary) -> Self {
        self.secretary = Some(value);
        self
    }

    pub fn members(mut self, value: Vec<IeB1GenerateDeclarationsResponseMembersItem>) -> Self {
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

    /// Consumes the builder and constructs a [`IeB1GenerateDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](IeB1GenerateDeclarationsResponseBuilder::year)
    /// - [`cro_number`](IeB1GenerateDeclarationsResponseBuilder::cro_number)
    /// - [`company_name`](IeB1GenerateDeclarationsResponseBuilder::company_name)
    /// - [`file_name`](IeB1GenerateDeclarationsResponseBuilder::file_name)
    /// - [`xml`](IeB1GenerateDeclarationsResponseBuilder::xml)
    /// - [`fields`](IeB1GenerateDeclarationsResponseBuilder::fields)
    /// - [`directors`](IeB1GenerateDeclarationsResponseBuilder::directors)
    /// - [`members`](IeB1GenerateDeclarationsResponseBuilder::members)
    /// - [`warnings`](IeB1GenerateDeclarationsResponseBuilder::warnings)
    /// - [`notes`](IeB1GenerateDeclarationsResponseBuilder::notes)
    /// - [`source`](IeB1GenerateDeclarationsResponseBuilder::source)
    pub fn build(self) -> Result<IeB1GenerateDeclarationsResponse, BuildError> {
        Ok(IeB1GenerateDeclarationsResponse {
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
