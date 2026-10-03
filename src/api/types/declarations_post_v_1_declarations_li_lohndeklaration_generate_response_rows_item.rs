pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(default)]
    pub versichertennummer: String,
    #[serde(default)]
    pub vorname: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub geschlecht: String,
    #[serde(default)]
    pub heimatstaat: String,
    #[serde(default)]
    pub eintrittsdatum: String,
    #[serde(default)]
    pub austrittsdatum: String,
    #[serde(rename = "beschaeftigtVon")]
    #[serde(default)]
    pub beschaeftigt_von: String,
    #[serde(rename = "beschaeftigtBis")]
    #[serde(default)]
    pub beschaeftigt_bis: String,
    #[serde(default)]
    pub beschaeftigungsgrad: String,
    #[serde(rename = "ahvLohn")]
    #[serde(default)]
    pub ahv_lohn: String,
    #[serde(default)]
    pub alv: String,
    #[serde(default)]
    pub warnings: Vec<String>,
}

impl PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItem {
    pub fn builder() -> PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder {
        <PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder {
    employee_id: Option<String>,
    versichertennummer: Option<String>,
    vorname: Option<String>,
    name: Option<String>,
    geschlecht: Option<String>,
    heimatstaat: Option<String>,
    eintrittsdatum: Option<String>,
    austrittsdatum: Option<String>,
    beschaeftigt_von: Option<String>,
    beschaeftigt_bis: Option<String>,
    beschaeftigungsgrad: Option<String>,
    ahv_lohn: Option<String>,
    alv: Option<String>,
    warnings: Option<Vec<String>>,
}

impl PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn versichertennummer(mut self, value: impl Into<String>) -> Self {
        self.versichertennummer = Some(value.into());
        self
    }

    pub fn vorname(mut self, value: impl Into<String>) -> Self {
        self.vorname = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn geschlecht(mut self, value: impl Into<String>) -> Self {
        self.geschlecht = Some(value.into());
        self
    }

    pub fn heimatstaat(mut self, value: impl Into<String>) -> Self {
        self.heimatstaat = Some(value.into());
        self
    }

    pub fn eintrittsdatum(mut self, value: impl Into<String>) -> Self {
        self.eintrittsdatum = Some(value.into());
        self
    }

    pub fn austrittsdatum(mut self, value: impl Into<String>) -> Self {
        self.austrittsdatum = Some(value.into());
        self
    }

    pub fn beschaeftigt_von(mut self, value: impl Into<String>) -> Self {
        self.beschaeftigt_von = Some(value.into());
        self
    }

    pub fn beschaeftigt_bis(mut self, value: impl Into<String>) -> Self {
        self.beschaeftigt_bis = Some(value.into());
        self
    }

    pub fn beschaeftigungsgrad(mut self, value: impl Into<String>) -> Self {
        self.beschaeftigungsgrad = Some(value.into());
        self
    }

    pub fn ahv_lohn(mut self, value: impl Into<String>) -> Self {
        self.ahv_lohn = Some(value.into());
        self
    }

    pub fn alv(mut self, value: impl Into<String>) -> Self {
        self.alv = Some(value.into());
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder::employee_id)
    /// - [`versichertennummer`](PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder::versichertennummer)
    /// - [`vorname`](PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder::vorname)
    /// - [`name`](PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder::name)
    /// - [`geschlecht`](PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder::geschlecht)
    /// - [`heimatstaat`](PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder::heimatstaat)
    /// - [`eintrittsdatum`](PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder::eintrittsdatum)
    /// - [`austrittsdatum`](PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder::austrittsdatum)
    /// - [`beschaeftigt_von`](PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder::beschaeftigt_von)
    /// - [`beschaeftigt_bis`](PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder::beschaeftigt_bis)
    /// - [`beschaeftigungsgrad`](PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder::beschaeftigungsgrad)
    /// - [`ahv_lohn`](PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder::ahv_lohn)
    /// - [`alv`](PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder::alv)
    /// - [`warnings`](PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItemBuilder::warnings)
    pub fn build(
        self,
    ) -> Result<PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItem, BuildError> {
        Ok(
            PostV1DeclarationsLiLohndeklarationGenerateResponseRowsItem {
                employee_id: self
                    .employee_id
                    .ok_or_else(|| BuildError::missing_field("employee_id"))?,
                versichertennummer: self
                    .versichertennummer
                    .ok_or_else(|| BuildError::missing_field("versichertennummer"))?,
                vorname: self
                    .vorname
                    .ok_or_else(|| BuildError::missing_field("vorname"))?,
                name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
                geschlecht: self
                    .geschlecht
                    .ok_or_else(|| BuildError::missing_field("geschlecht"))?,
                heimatstaat: self
                    .heimatstaat
                    .ok_or_else(|| BuildError::missing_field("heimatstaat"))?,
                eintrittsdatum: self
                    .eintrittsdatum
                    .ok_or_else(|| BuildError::missing_field("eintrittsdatum"))?,
                austrittsdatum: self
                    .austrittsdatum
                    .ok_or_else(|| BuildError::missing_field("austrittsdatum"))?,
                beschaeftigt_von: self
                    .beschaeftigt_von
                    .ok_or_else(|| BuildError::missing_field("beschaeftigt_von"))?,
                beschaeftigt_bis: self
                    .beschaeftigt_bis
                    .ok_or_else(|| BuildError::missing_field("beschaeftigt_bis"))?,
                beschaeftigungsgrad: self
                    .beschaeftigungsgrad
                    .ok_or_else(|| BuildError::missing_field("beschaeftigungsgrad"))?,
                ahv_lohn: self
                    .ahv_lohn
                    .ok_or_else(|| BuildError::missing_field("ahv_lohn"))?,
                alv: self.alv.ok_or_else(|| BuildError::missing_field("alv"))?,
                warnings: self
                    .warnings
                    .ok_or_else(|| BuildError::missing_field("warnings"))?,
            },
        )
    }
}
