pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsPlPit11GenerateResponsePersonsItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pesel: Option<String>,
    #[serde(default)]
    pub revenue: String,
    #[serde(rename = "deductibleCosts")]
    #[serde(default)]
    pub deductible_costs: String,
    #[serde(rename = "advanceWithheld")]
    #[serde(default)]
    pub advance_withheld: String,
    #[serde(rename = "socialContributions")]
    #[serde(default)]
    pub social_contributions: String,
    #[serde(rename = "healthContributions")]
    #[serde(default)]
    pub health_contributions: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(default)]
    pub xml: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1DeclarationsPlPit11GenerateResponsePersonsItem {
    pub fn builder() -> PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder {
        <PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder {
    employee_id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    pesel: Option<String>,
    revenue: Option<String>,
    deductible_costs: Option<String>,
    advance_withheld: Option<String>,
    social_contributions: Option<String>,
    health_contributions: Option<String>,
    file_name: Option<String>,
    xml: Option<String>,
    warnings: Option<Vec<String>>,
}

impl PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn pesel(mut self, value: impl Into<String>) -> Self {
        self.pesel = Some(value.into());
        self
    }

    pub fn revenue(mut self, value: impl Into<String>) -> Self {
        self.revenue = Some(value.into());
        self
    }

    pub fn deductible_costs(mut self, value: impl Into<String>) -> Self {
        self.deductible_costs = Some(value.into());
        self
    }

    pub fn advance_withheld(mut self, value: impl Into<String>) -> Self {
        self.advance_withheld = Some(value.into());
        self
    }

    pub fn social_contributions(mut self, value: impl Into<String>) -> Self {
        self.social_contributions = Some(value.into());
        self
    }

    pub fn health_contributions(mut self, value: impl Into<String>) -> Self {
        self.health_contributions = Some(value.into());
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

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsPlPit11GenerateResponsePersonsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder::employee_id)
    /// - [`first_name`](PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder::first_name)
    /// - [`last_name`](PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder::last_name)
    /// - [`revenue`](PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder::revenue)
    /// - [`deductible_costs`](PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder::deductible_costs)
    /// - [`advance_withheld`](PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder::advance_withheld)
    /// - [`social_contributions`](PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder::social_contributions)
    /// - [`health_contributions`](PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder::health_contributions)
    /// - [`file_name`](PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder::file_name)
    /// - [`xml`](PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder::xml)
    /// - [`warnings`](PostV1DeclarationsPlPit11GenerateResponsePersonsItemBuilder::warnings)
    pub fn build(self) -> Result<PostV1DeclarationsPlPit11GenerateResponsePersonsItem, BuildError> {
        Ok(PostV1DeclarationsPlPit11GenerateResponsePersonsItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            pesel: self.pesel,
            revenue: self
                .revenue
                .ok_or_else(|| BuildError::missing_field("revenue"))?,
            deductible_costs: self
                .deductible_costs
                .ok_or_else(|| BuildError::missing_field("deductible_costs"))?,
            advance_withheld: self
                .advance_withheld
                .ok_or_else(|| BuildError::missing_field("advance_withheld"))?,
            social_contributions: self
                .social_contributions
                .ok_or_else(|| BuildError::missing_field("social_contributions"))?,
            health_contributions: self
                .health_contributions
                .ok_or_else(|| BuildError::missing_field("health_contributions"))?,
            file_name: self
                .file_name
                .ok_or_else(|| BuildError::missing_field("file_name"))?,
            xml: self.xml.ok_or_else(|| BuildError::missing_field("xml"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
