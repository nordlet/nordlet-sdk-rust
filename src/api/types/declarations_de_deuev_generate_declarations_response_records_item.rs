pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeDeuevGenerateDeclarationsResponseRecordsItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub abgabegrund: String,
    #[serde(default)]
    pub versicherungsnummer: String,
    #[serde(rename = "betriebsnummerKrankenkasse")]
    #[serde(default)]
    pub betriebsnummer_krankenkasse: String,
    #[serde(default)]
    pub personengruppe: String,
    #[serde(default)]
    pub beitragsgruppe: String,
    #[serde(rename = "zeitraumBeginn")]
    #[serde(default)]
    pub zeitraum_beginn: String,
    #[serde(rename = "zeitraumEnde")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zeitraum_ende: Option<String>,
    #[serde(default)]
    pub entgelt: String,
    #[serde(default)]
    pub record: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl DeDeuevGenerateDeclarationsResponseRecordsItem {
    pub fn builder() -> DeDeuevGenerateDeclarationsResponseRecordsItemBuilder {
        <DeDeuevGenerateDeclarationsResponseRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeDeuevGenerateDeclarationsResponseRecordsItemBuilder {
    employee_id: Option<String>,
    name: Option<String>,
    abgabegrund: Option<String>,
    versicherungsnummer: Option<String>,
    betriebsnummer_krankenkasse: Option<String>,
    personengruppe: Option<String>,
    beitragsgruppe: Option<String>,
    zeitraum_beginn: Option<String>,
    zeitraum_ende: Option<String>,
    entgelt: Option<String>,
    record: Option<String>,
    warnings: Option<Vec<String>>,
}

impl DeDeuevGenerateDeclarationsResponseRecordsItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn abgabegrund(mut self, value: impl Into<String>) -> Self {
        self.abgabegrund = Some(value.into());
        self
    }

    pub fn versicherungsnummer(mut self, value: impl Into<String>) -> Self {
        self.versicherungsnummer = Some(value.into());
        self
    }

    pub fn betriebsnummer_krankenkasse(mut self, value: impl Into<String>) -> Self {
        self.betriebsnummer_krankenkasse = Some(value.into());
        self
    }

    pub fn personengruppe(mut self, value: impl Into<String>) -> Self {
        self.personengruppe = Some(value.into());
        self
    }

    pub fn beitragsgruppe(mut self, value: impl Into<String>) -> Self {
        self.beitragsgruppe = Some(value.into());
        self
    }

    pub fn zeitraum_beginn(mut self, value: impl Into<String>) -> Self {
        self.zeitraum_beginn = Some(value.into());
        self
    }

    pub fn zeitraum_ende(mut self, value: impl Into<String>) -> Self {
        self.zeitraum_ende = Some(value.into());
        self
    }

    pub fn entgelt(mut self, value: impl Into<String>) -> Self {
        self.entgelt = Some(value.into());
        self
    }

    pub fn record(mut self, value: impl Into<String>) -> Self {
        self.record = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeDeuevGenerateDeclarationsResponseRecordsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](DeDeuevGenerateDeclarationsResponseRecordsItemBuilder::employee_id)
    /// - [`name`](DeDeuevGenerateDeclarationsResponseRecordsItemBuilder::name)
    /// - [`abgabegrund`](DeDeuevGenerateDeclarationsResponseRecordsItemBuilder::abgabegrund)
    /// - [`versicherungsnummer`](DeDeuevGenerateDeclarationsResponseRecordsItemBuilder::versicherungsnummer)
    /// - [`betriebsnummer_krankenkasse`](DeDeuevGenerateDeclarationsResponseRecordsItemBuilder::betriebsnummer_krankenkasse)
    /// - [`personengruppe`](DeDeuevGenerateDeclarationsResponseRecordsItemBuilder::personengruppe)
    /// - [`beitragsgruppe`](DeDeuevGenerateDeclarationsResponseRecordsItemBuilder::beitragsgruppe)
    /// - [`zeitraum_beginn`](DeDeuevGenerateDeclarationsResponseRecordsItemBuilder::zeitraum_beginn)
    /// - [`entgelt`](DeDeuevGenerateDeclarationsResponseRecordsItemBuilder::entgelt)
    /// - [`record`](DeDeuevGenerateDeclarationsResponseRecordsItemBuilder::record)
    /// - [`warnings`](DeDeuevGenerateDeclarationsResponseRecordsItemBuilder::warnings)
    pub fn build(self) -> Result<DeDeuevGenerateDeclarationsResponseRecordsItem, BuildError> {
        Ok(DeDeuevGenerateDeclarationsResponseRecordsItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            abgabegrund: self
                .abgabegrund
                .ok_or_else(|| BuildError::missing_field("abgabegrund"))?,
            versicherungsnummer: self
                .versicherungsnummer
                .ok_or_else(|| BuildError::missing_field("versicherungsnummer"))?,
            betriebsnummer_krankenkasse: self
                .betriebsnummer_krankenkasse
                .ok_or_else(|| BuildError::missing_field("betriebsnummer_krankenkasse"))?,
            personengruppe: self
                .personengruppe
                .ok_or_else(|| BuildError::missing_field("personengruppe"))?,
            beitragsgruppe: self
                .beitragsgruppe
                .ok_or_else(|| BuildError::missing_field("beitragsgruppe"))?,
            zeitraum_beginn: self
                .zeitraum_beginn
                .ok_or_else(|| BuildError::missing_field("zeitraum_beginn"))?,
            zeitraum_ende: self.zeitraum_ende,
            entgelt: self
                .entgelt
                .ok_or_else(|| BuildError::missing_field("entgelt"))?,
            record: self
                .record
                .ok_or_else(|| BuildError::missing_field("record"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
