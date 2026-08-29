pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostV1ProductionRoutingsGetResponseOperationsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub sequence: i64,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "workCenterId")]
    #[serde(default)]
    pub work_center_id: String,
    #[serde(rename = "setupMinutes")]
    #[serde(default)]
    pub setup_minutes: String,
    #[serde(rename = "runMinutesPerUnit")]
    #[serde(default)]
    pub run_minutes_per_unit: String,
    #[serde(rename = "qualityCheckName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_check_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl PostV1ProductionRoutingsGetResponseOperationsItem {
    pub fn builder() -> PostV1ProductionRoutingsGetResponseOperationsItemBuilder {
        <PostV1ProductionRoutingsGetResponseOperationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostV1ProductionRoutingsGetResponseOperationsItemBuilder {
    id: Option<String>,
    sequence: Option<i64>,
    name: Option<String>,
    work_center_id: Option<String>,
    setup_minutes: Option<String>,
    run_minutes_per_unit: Option<String>,
    quality_check_name: Option<String>,
    notes: Option<String>,
}

impl PostV1ProductionRoutingsGetResponseOperationsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn work_center_id(mut self, value: impl Into<String>) -> Self {
        self.work_center_id = Some(value.into());
        self
    }

    pub fn setup_minutes(mut self, value: impl Into<String>) -> Self {
        self.setup_minutes = Some(value.into());
        self
    }

    pub fn run_minutes_per_unit(mut self, value: impl Into<String>) -> Self {
        self.run_minutes_per_unit = Some(value.into());
        self
    }

    pub fn quality_check_name(mut self, value: impl Into<String>) -> Self {
        self.quality_check_name = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostV1ProductionRoutingsGetResponseOperationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostV1ProductionRoutingsGetResponseOperationsItemBuilder::id)
    /// - [`sequence`](PostV1ProductionRoutingsGetResponseOperationsItemBuilder::sequence)
    /// - [`name`](PostV1ProductionRoutingsGetResponseOperationsItemBuilder::name)
    /// - [`work_center_id`](PostV1ProductionRoutingsGetResponseOperationsItemBuilder::work_center_id)
    /// - [`setup_minutes`](PostV1ProductionRoutingsGetResponseOperationsItemBuilder::setup_minutes)
    /// - [`run_minutes_per_unit`](PostV1ProductionRoutingsGetResponseOperationsItemBuilder::run_minutes_per_unit)
    pub fn build(self) -> Result<PostV1ProductionRoutingsGetResponseOperationsItem, BuildError> {
        Ok(PostV1ProductionRoutingsGetResponseOperationsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            sequence: self
                .sequence
                .ok_or_else(|| BuildError::missing_field("sequence"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            work_center_id: self
                .work_center_id
                .ok_or_else(|| BuildError::missing_field("work_center_id"))?,
            setup_minutes: self
                .setup_minutes
                .ok_or_else(|| BuildError::missing_field("setup_minutes"))?,
            run_minutes_per_unit: self
                .run_minutes_per_unit
                .ok_or_else(|| BuildError::missing_field("run_minutes_per_unit"))?,
            quality_check_name: self.quality_check_name,
            notes: self.notes,
        })
    }
}
