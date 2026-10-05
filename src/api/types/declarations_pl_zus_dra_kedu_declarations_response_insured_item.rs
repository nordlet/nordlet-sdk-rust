pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlZusDraKeduDeclarationsResponseInsuredItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub pesel: String,
    #[serde(rename = "kodTytulu")]
    #[serde(default)]
    pub kod_tytulu: PlZusDraKeduDeclarationsResponseInsuredItemKodTytulu,
    #[serde(rename = "pensionBase")]
    #[serde(default)]
    pub pension_base: String,
    #[serde(rename = "healthBase")]
    #[serde(default)]
    pub health_base: String,
}

impl PlZusDraKeduDeclarationsResponseInsuredItem {
    pub fn builder() -> PlZusDraKeduDeclarationsResponseInsuredItemBuilder {
        <PlZusDraKeduDeclarationsResponseInsuredItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlZusDraKeduDeclarationsResponseInsuredItemBuilder {
    employee_id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    pesel: Option<String>,
    kod_tytulu: Option<PlZusDraKeduDeclarationsResponseInsuredItemKodTytulu>,
    pension_base: Option<String>,
    health_base: Option<String>,
}

impl PlZusDraKeduDeclarationsResponseInsuredItemBuilder {
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

    pub fn kod_tytulu(
        mut self,
        value: PlZusDraKeduDeclarationsResponseInsuredItemKodTytulu,
    ) -> Self {
        self.kod_tytulu = Some(value);
        self
    }

    pub fn pension_base(mut self, value: impl Into<String>) -> Self {
        self.pension_base = Some(value.into());
        self
    }

    pub fn health_base(mut self, value: impl Into<String>) -> Self {
        self.health_base = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PlZusDraKeduDeclarationsResponseInsuredItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PlZusDraKeduDeclarationsResponseInsuredItemBuilder::employee_id)
    /// - [`first_name`](PlZusDraKeduDeclarationsResponseInsuredItemBuilder::first_name)
    /// - [`last_name`](PlZusDraKeduDeclarationsResponseInsuredItemBuilder::last_name)
    /// - [`pesel`](PlZusDraKeduDeclarationsResponseInsuredItemBuilder::pesel)
    /// - [`kod_tytulu`](PlZusDraKeduDeclarationsResponseInsuredItemBuilder::kod_tytulu)
    /// - [`pension_base`](PlZusDraKeduDeclarationsResponseInsuredItemBuilder::pension_base)
    /// - [`health_base`](PlZusDraKeduDeclarationsResponseInsuredItemBuilder::health_base)
    pub fn build(self) -> Result<PlZusDraKeduDeclarationsResponseInsuredItem, BuildError> {
        Ok(PlZusDraKeduDeclarationsResponseInsuredItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            pesel: self
                .pesel
                .ok_or_else(|| BuildError::missing_field("pesel"))?,
            kod_tytulu: self
                .kod_tytulu
                .ok_or_else(|| BuildError::missing_field("kod_tytulu"))?,
            pension_base: self
                .pension_base
                .ok_or_else(|| BuildError::missing_field("pension_base"))?,
            health_base: self
                .health_base
                .ok_or_else(|| BuildError::missing_field("health_base"))?,
        })
    }
}
