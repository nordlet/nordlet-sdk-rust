pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LiLohnlistenGenerateDeclarationsResponseRowsItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub peid: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub vorname: String,
    #[serde(default)]
    pub geburtsdatum: String,
    #[serde(default)]
    pub strasse: String,
    #[serde(default)]
    pub hausnummer: String,
    #[serde(default)]
    pub plz: String,
    #[serde(default)]
    pub ort: String,
    #[serde(default)]
    pub wohnland: String,
    #[serde(default)]
    pub brutto: String,
    #[serde(default)]
    pub lohnsteuer: String,
    #[serde(rename = "abrechnungVon")]
    #[serde(default)]
    pub abrechnung_von: String,
    #[serde(rename = "abrechnungBis")]
    #[serde(default)]
    pub abrechnung_bis: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl LiLohnlistenGenerateDeclarationsResponseRowsItem {
    pub fn builder() -> LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder {
        <LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder {
    employee_id: Option<String>,
    peid: Option<String>,
    name: Option<String>,
    vorname: Option<String>,
    geburtsdatum: Option<String>,
    strasse: Option<String>,
    hausnummer: Option<String>,
    plz: Option<String>,
    ort: Option<String>,
    wohnland: Option<String>,
    brutto: Option<String>,
    lohnsteuer: Option<String>,
    abrechnung_von: Option<String>,
    abrechnung_bis: Option<String>,
    warnings: Option<Vec<String>>,
}

impl LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn peid(mut self, value: impl Into<String>) -> Self {
        self.peid = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn vorname(mut self, value: impl Into<String>) -> Self {
        self.vorname = Some(value.into());
        self
    }

    pub fn geburtsdatum(mut self, value: impl Into<String>) -> Self {
        self.geburtsdatum = Some(value.into());
        self
    }

    pub fn strasse(mut self, value: impl Into<String>) -> Self {
        self.strasse = Some(value.into());
        self
    }

    pub fn hausnummer(mut self, value: impl Into<String>) -> Self {
        self.hausnummer = Some(value.into());
        self
    }

    pub fn plz(mut self, value: impl Into<String>) -> Self {
        self.plz = Some(value.into());
        self
    }

    pub fn ort(mut self, value: impl Into<String>) -> Self {
        self.ort = Some(value.into());
        self
    }

    pub fn wohnland(mut self, value: impl Into<String>) -> Self {
        self.wohnland = Some(value.into());
        self
    }

    pub fn brutto(mut self, value: impl Into<String>) -> Self {
        self.brutto = Some(value.into());
        self
    }

    pub fn lohnsteuer(mut self, value: impl Into<String>) -> Self {
        self.lohnsteuer = Some(value.into());
        self
    }

    pub fn abrechnung_von(mut self, value: impl Into<String>) -> Self {
        self.abrechnung_von = Some(value.into());
        self
    }

    pub fn abrechnung_bis(mut self, value: impl Into<String>) -> Self {
        self.abrechnung_bis = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LiLohnlistenGenerateDeclarationsResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::employee_id)
    /// - [`peid`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::peid)
    /// - [`name`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::name)
    /// - [`vorname`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::vorname)
    /// - [`geburtsdatum`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::geburtsdatum)
    /// - [`strasse`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::strasse)
    /// - [`hausnummer`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::hausnummer)
    /// - [`plz`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::plz)
    /// - [`ort`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::ort)
    /// - [`wohnland`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::wohnland)
    /// - [`brutto`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::brutto)
    /// - [`lohnsteuer`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::lohnsteuer)
    /// - [`abrechnung_von`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::abrechnung_von)
    /// - [`abrechnung_bis`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::abrechnung_bis)
    /// - [`warnings`](LiLohnlistenGenerateDeclarationsResponseRowsItemBuilder::warnings)
    pub fn build(self) -> Result<LiLohnlistenGenerateDeclarationsResponseRowsItem, BuildError> {
        Ok(LiLohnlistenGenerateDeclarationsResponseRowsItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            peid: self.peid.ok_or_else(|| BuildError::missing_field("peid"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            vorname: self
                .vorname
                .ok_or_else(|| BuildError::missing_field("vorname"))?,
            geburtsdatum: self
                .geburtsdatum
                .ok_or_else(|| BuildError::missing_field("geburtsdatum"))?,
            strasse: self
                .strasse
                .ok_or_else(|| BuildError::missing_field("strasse"))?,
            hausnummer: self
                .hausnummer
                .ok_or_else(|| BuildError::missing_field("hausnummer"))?,
            plz: self.plz.ok_or_else(|| BuildError::missing_field("plz"))?,
            ort: self.ort.ok_or_else(|| BuildError::missing_field("ort"))?,
            wohnland: self
                .wohnland
                .ok_or_else(|| BuildError::missing_field("wohnland"))?,
            brutto: self
                .brutto
                .ok_or_else(|| BuildError::missing_field("brutto"))?,
            lohnsteuer: self
                .lohnsteuer
                .ok_or_else(|| BuildError::missing_field("lohnsteuer"))?,
            abrechnung_von: self
                .abrechnung_von
                .ok_or_else(|| BuildError::missing_field("abrechnung_von"))?,
            abrechnung_bis: self
                .abrechnung_bis
                .ok_or_else(|| BuildError::missing_field("abrechnung_bis"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
        })
    }
}
