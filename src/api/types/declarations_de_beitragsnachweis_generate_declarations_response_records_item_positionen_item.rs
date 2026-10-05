pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemPositionenItem {
    #[serde(default)]
    pub beitragsgruppe: String,
    #[serde(default)]
    pub betrag: String,
}

impl DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemPositionenItem {
    pub fn builder(
    ) -> DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemPositionenItemBuilder {
        <DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemPositionenItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemPositionenItemBuilder {
    beitragsgruppe: Option<String>,
    betrag: Option<String>,
}

impl DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemPositionenItemBuilder {
    pub fn beitragsgruppe(mut self, value: impl Into<String>) -> Self {
        self.beitragsgruppe = Some(value.into());
        self
    }

    pub fn betrag(mut self, value: impl Into<String>) -> Self {
        self.betrag = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemPositionenItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`beitragsgruppe`](DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemPositionenItemBuilder::beitragsgruppe)
    /// - [`betrag`](DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemPositionenItemBuilder::betrag)
    pub fn build(
        self,
    ) -> Result<DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemPositionenItem, BuildError>
    {
        Ok(
            DeBeitragsnachweisGenerateDeclarationsResponseRecordsItemPositionenItem {
                beitragsgruppe: self
                    .beitragsgruppe
                    .ok_or_else(|| BuildError::missing_field("beitragsgruppe"))?,
                betrag: self
                    .betrag
                    .ok_or_else(|| BuildError::missing_field("betrag"))?,
            },
        )
    }
}
