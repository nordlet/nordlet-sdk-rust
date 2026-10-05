pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeReturnFactsSetDeclarationsRequestFactsMunicipalitiesItem {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "postalCode")]
    #[serde(default)]
    pub postal_code: String,
    #[serde(default)]
    pub ags: String,
    #[serde(default)]
    pub hebesatz: String,
    #[serde(default)]
    pub wages: String,
}

impl DeReturnFactsSetDeclarationsRequestFactsMunicipalitiesItem {
    pub fn builder() -> DeReturnFactsSetDeclarationsRequestFactsMunicipalitiesItemBuilder {
        <DeReturnFactsSetDeclarationsRequestFactsMunicipalitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeReturnFactsSetDeclarationsRequestFactsMunicipalitiesItemBuilder {
    name: Option<String>,
    postal_code: Option<String>,
    ags: Option<String>,
    hebesatz: Option<String>,
    wages: Option<String>,
}

impl DeReturnFactsSetDeclarationsRequestFactsMunicipalitiesItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn ags(mut self, value: impl Into<String>) -> Self {
        self.ags = Some(value.into());
        self
    }

    pub fn hebesatz(mut self, value: impl Into<String>) -> Self {
        self.hebesatz = Some(value.into());
        self
    }

    pub fn wages(mut self, value: impl Into<String>) -> Self {
        self.wages = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeReturnFactsSetDeclarationsRequestFactsMunicipalitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DeReturnFactsSetDeclarationsRequestFactsMunicipalitiesItemBuilder::name)
    /// - [`postal_code`](DeReturnFactsSetDeclarationsRequestFactsMunicipalitiesItemBuilder::postal_code)
    /// - [`ags`](DeReturnFactsSetDeclarationsRequestFactsMunicipalitiesItemBuilder::ags)
    /// - [`hebesatz`](DeReturnFactsSetDeclarationsRequestFactsMunicipalitiesItemBuilder::hebesatz)
    /// - [`wages`](DeReturnFactsSetDeclarationsRequestFactsMunicipalitiesItemBuilder::wages)
    pub fn build(
        self,
    ) -> Result<DeReturnFactsSetDeclarationsRequestFactsMunicipalitiesItem, BuildError> {
        Ok(DeReturnFactsSetDeclarationsRequestFactsMunicipalitiesItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            postal_code: self
                .postal_code
                .ok_or_else(|| BuildError::missing_field("postal_code"))?,
            ags: self.ags.ok_or_else(|| BuildError::missing_field("ags"))?,
            hebesatz: self
                .hebesatz
                .ok_or_else(|| BuildError::missing_field("hebesatz"))?,
            wages: self
                .wages
                .ok_or_else(|| BuildError::missing_field("wages"))?,
        })
    }
}
