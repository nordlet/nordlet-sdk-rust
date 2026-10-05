pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlPit11GenerateDeclarationsResponsePersonsItem {
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

impl PlPit11GenerateDeclarationsResponsePersonsItem {
    pub fn builder() -> PlPit11GenerateDeclarationsResponsePersonsItemBuilder {
        <PlPit11GenerateDeclarationsResponsePersonsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlPit11GenerateDeclarationsResponsePersonsItemBuilder {
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

impl PlPit11GenerateDeclarationsResponsePersonsItemBuilder {
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

    /// Consumes the builder and constructs a [`PlPit11GenerateDeclarationsResponsePersonsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PlPit11GenerateDeclarationsResponsePersonsItemBuilder::employee_id)
    /// - [`first_name`](PlPit11GenerateDeclarationsResponsePersonsItemBuilder::first_name)
    /// - [`last_name`](PlPit11GenerateDeclarationsResponsePersonsItemBuilder::last_name)
    /// - [`revenue`](PlPit11GenerateDeclarationsResponsePersonsItemBuilder::revenue)
    /// - [`deductible_costs`](PlPit11GenerateDeclarationsResponsePersonsItemBuilder::deductible_costs)
    /// - [`advance_withheld`](PlPit11GenerateDeclarationsResponsePersonsItemBuilder::advance_withheld)
    /// - [`social_contributions`](PlPit11GenerateDeclarationsResponsePersonsItemBuilder::social_contributions)
    /// - [`health_contributions`](PlPit11GenerateDeclarationsResponsePersonsItemBuilder::health_contributions)
    /// - [`file_name`](PlPit11GenerateDeclarationsResponsePersonsItemBuilder::file_name)
    /// - [`xml`](PlPit11GenerateDeclarationsResponsePersonsItemBuilder::xml)
    /// - [`warnings`](PlPit11GenerateDeclarationsResponsePersonsItemBuilder::warnings)
    pub fn build(self) -> Result<PlPit11GenerateDeclarationsResponsePersonsItem, BuildError> {
        Ok(PlPit11GenerateDeclarationsResponsePersonsItem {
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
