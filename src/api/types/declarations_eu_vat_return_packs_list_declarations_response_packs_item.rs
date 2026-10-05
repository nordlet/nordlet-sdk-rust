pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EuVatReturnPacksListDeclarationsResponsePacksItem {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "formKey")]
    #[serde(default)]
    pub form_key: String,
    #[serde(rename = "formName")]
    #[serde(default)]
    pub form_name: String,
    pub frequency: EuVatReturnPacksListDeclarationsResponsePacksItemFrequency,
    #[serde(default)]
    pub source: String,
}

impl EuVatReturnPacksListDeclarationsResponsePacksItem {
    pub fn builder() -> EuVatReturnPacksListDeclarationsResponsePacksItemBuilder {
        <EuVatReturnPacksListDeclarationsResponsePacksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EuVatReturnPacksListDeclarationsResponsePacksItemBuilder {
    country_code: Option<String>,
    form_key: Option<String>,
    form_name: Option<String>,
    frequency: Option<EuVatReturnPacksListDeclarationsResponsePacksItemFrequency>,
    source: Option<String>,
}

impl EuVatReturnPacksListDeclarationsResponsePacksItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn form_key(mut self, value: impl Into<String>) -> Self {
        self.form_key = Some(value.into());
        self
    }

    pub fn form_name(mut self, value: impl Into<String>) -> Self {
        self.form_name = Some(value.into());
        self
    }

    pub fn frequency(
        mut self,
        value: EuVatReturnPacksListDeclarationsResponsePacksItemFrequency,
    ) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EuVatReturnPacksListDeclarationsResponsePacksItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`country_code`](EuVatReturnPacksListDeclarationsResponsePacksItemBuilder::country_code)
    /// - [`form_key`](EuVatReturnPacksListDeclarationsResponsePacksItemBuilder::form_key)
    /// - [`form_name`](EuVatReturnPacksListDeclarationsResponsePacksItemBuilder::form_name)
    /// - [`frequency`](EuVatReturnPacksListDeclarationsResponsePacksItemBuilder::frequency)
    /// - [`source`](EuVatReturnPacksListDeclarationsResponsePacksItemBuilder::source)
    pub fn build(self) -> Result<EuVatReturnPacksListDeclarationsResponsePacksItem, BuildError> {
        Ok(EuVatReturnPacksListDeclarationsResponsePacksItem {
            country_code: self
                .country_code
                .ok_or_else(|| BuildError::missing_field("country_code"))?,
            form_key: self
                .form_key
                .ok_or_else(|| BuildError::missing_field("form_key"))?,
            form_name: self
                .form_name
                .ok_or_else(|| BuildError::missing_field("form_name"))?,
            frequency: self
                .frequency
                .ok_or_else(|| BuildError::missing_field("frequency"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
