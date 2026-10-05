pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeBeitragsnachweisGenerateDeclarationsResponseRecordsItem {
    #[serde(rename = "betriebsnummerKrankenkasse")]
    #[serde(default)]
    pub betriebsnummer_krankenkasse: String,
    #[serde(default)]
    pub faelligkeitstag: String,
    #[serde(rename = "kvAllgemein")]
    #[serde(default)]
    pub kv_allgemein: String,
    #[serde(rename = "kvZusatzbeitrag")]
    #[serde(default)]
    pub kv_zusatzbeitrag: String,
    #[serde(default)]
    pub pauschsteuer: String,
    #[serde(rename = "beitragssatzAllgemein")]
    #[serde(default)]
    pub beitragssatz_allgemein: String,
    #[serde(default)]
    pub summe: String,
    #[serde(default)]
    pub positionen: Vec<DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemPositionenItem>,
    #[serde(default)]
    pub record: String,
}

impl DeBeitragsnachweisGenerateDeclarationsResponseRecordsItem {
    pub fn builder() -> DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemBuilder {
        <DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemBuilder {
    betriebsnummer_krankenkasse: Option<String>,
    faelligkeitstag: Option<String>,
    kv_allgemein: Option<String>,
    kv_zusatzbeitrag: Option<String>,
    pauschsteuer: Option<String>,
    beitragssatz_allgemein: Option<String>,
    summe: Option<String>,
    positionen:
        Option<Vec<DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemPositionenItem>>,
    record: Option<String>,
}

impl DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemBuilder {
    pub fn betriebsnummer_krankenkasse(mut self, value: impl Into<String>) -> Self {
        self.betriebsnummer_krankenkasse = Some(value.into());
        self
    }

    pub fn faelligkeitstag(mut self, value: impl Into<String>) -> Self {
        self.faelligkeitstag = Some(value.into());
        self
    }

    pub fn kv_allgemein(mut self, value: impl Into<String>) -> Self {
        self.kv_allgemein = Some(value.into());
        self
    }

    pub fn kv_zusatzbeitrag(mut self, value: impl Into<String>) -> Self {
        self.kv_zusatzbeitrag = Some(value.into());
        self
    }

    pub fn pauschsteuer(mut self, value: impl Into<String>) -> Self {
        self.pauschsteuer = Some(value.into());
        self
    }

    pub fn beitragssatz_allgemein(mut self, value: impl Into<String>) -> Self {
        self.beitragssatz_allgemein = Some(value.into());
        self
    }

    pub fn summe(mut self, value: impl Into<String>) -> Self {
        self.summe = Some(value.into());
        self
    }

    pub fn positionen(
        mut self,
        value: Vec<DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemPositionenItem>,
    ) -> Self {
        self.positionen = Some(value);
        self
    }

    pub fn record(mut self, value: impl Into<String>) -> Self {
        self.record = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeBeitragsnachweisGenerateDeclarationsResponseRecordsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`betriebsnummer_krankenkasse`](DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemBuilder::betriebsnummer_krankenkasse)
    /// - [`faelligkeitstag`](DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemBuilder::faelligkeitstag)
    /// - [`kv_allgemein`](DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemBuilder::kv_allgemein)
    /// - [`kv_zusatzbeitrag`](DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemBuilder::kv_zusatzbeitrag)
    /// - [`pauschsteuer`](DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemBuilder::pauschsteuer)
    /// - [`beitragssatz_allgemein`](DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemBuilder::beitragssatz_allgemein)
    /// - [`summe`](DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemBuilder::summe)
    /// - [`positionen`](DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemBuilder::positionen)
    /// - [`record`](DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemBuilder::record)
    pub fn build(
        self,
    ) -> Result<DeBeitragsnachweisGenerateDeclarationsResponseRecordsItem, BuildError> {
        Ok(DeBeitragsnachweisGenerateDeclarationsResponseRecordsItem {
            betriebsnummer_krankenkasse: self
                .betriebsnummer_krankenkasse
                .ok_or_else(|| BuildError::missing_field("betriebsnummer_krankenkasse"))?,
            faelligkeitstag: self
                .faelligkeitstag
                .ok_or_else(|| BuildError::missing_field("faelligkeitstag"))?,
            kv_allgemein: self
                .kv_allgemein
                .ok_or_else(|| BuildError::missing_field("kv_allgemein"))?,
            kv_zusatzbeitrag: self
                .kv_zusatzbeitrag
                .ok_or_else(|| BuildError::missing_field("kv_zusatzbeitrag"))?,
            pauschsteuer: self
                .pauschsteuer
                .ok_or_else(|| BuildError::missing_field("pauschsteuer"))?,
            beitragssatz_allgemein: self
                .beitragssatz_allgemein
                .ok_or_else(|| BuildError::missing_field("beitragssatz_allgemein"))?,
            summe: self
                .summe
                .ok_or_else(|| BuildError::missing_field("summe"))?,
            positionen: self
                .positionen
                .ok_or_else(|| BuildError::missing_field("positionen"))?,
            record: self
                .record
                .ok_or_else(|| BuildError::missing_field("record"))?,
        })
    }
}
