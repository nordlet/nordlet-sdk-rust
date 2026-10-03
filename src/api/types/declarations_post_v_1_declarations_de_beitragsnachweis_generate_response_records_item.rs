pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItem {
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
    pub positionen:
        Vec<PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemPositionenItem>,
    #[serde(default)]
    pub record: String,
}

impl PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItem {
    pub fn builder() -> PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemBuilder {
        <PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemBuilder as Default>::default(
        )
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemBuilder {
    betriebsnummer_krankenkasse: Option<String>,
    faelligkeitstag: Option<String>,
    kv_allgemein: Option<String>,
    kv_zusatzbeitrag: Option<String>,
    pauschsteuer: Option<String>,
    beitragssatz_allgemein: Option<String>,
    summe: Option<String>,
    positionen:
        Option<Vec<PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemPositionenItem>>,
    record: Option<String>,
}

impl PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemBuilder {
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
        value: Vec<PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemPositionenItem>,
    ) -> Self {
        self.positionen = Some(value);
        self
    }

    pub fn record(mut self, value: impl Into<String>) -> Self {
        self.record = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`betriebsnummer_krankenkasse`](PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemBuilder::betriebsnummer_krankenkasse)
    /// - [`faelligkeitstag`](PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemBuilder::faelligkeitstag)
    /// - [`kv_allgemein`](PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemBuilder::kv_allgemein)
    /// - [`kv_zusatzbeitrag`](PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemBuilder::kv_zusatzbeitrag)
    /// - [`pauschsteuer`](PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemBuilder::pauschsteuer)
    /// - [`beitragssatz_allgemein`](PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemBuilder::beitragssatz_allgemein)
    /// - [`summe`](PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemBuilder::summe)
    /// - [`positionen`](PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemBuilder::positionen)
    /// - [`record`](PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemBuilder::record)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItem, BuildError> {
        Ok(
            PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItem {
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
            },
        )
    }
}
