pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemPositionenItem {
    #[serde(default)]
    pub beitragsgruppe: String,
    #[serde(default)]
    pub betrag: String,
}

impl PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemPositionenItem {
    pub fn builder(
    ) -> PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemPositionenItemBuilder {
        <PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemPositionenItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemPositionenItemBuilder {
    beitragsgruppe: Option<String>,
    betrag: Option<String>,
}

impl PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemPositionenItemBuilder {
    pub fn beitragsgruppe(mut self, value: impl Into<String>) -> Self {
        self.beitragsgruppe = Some(value.into());
        self
    }

    pub fn betrag(mut self, value: impl Into<String>) -> Self {
        self.betrag = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemPositionenItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`beitragsgruppe`](PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemPositionenItemBuilder::beitragsgruppe)
    /// - [`betrag`](PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemPositionenItemBuilder::betrag)
    pub fn build(
        self,
    ) -> Result<
        PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemPositionenItem,
        BuildError,
    > {
        Ok(
            PostV1DeclarationsDeBeitragsnachweisGenerateResponseRecordsItemPositionenItem {
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
