pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DebtRemindersSettingsUpdatePartnersResponse {
    #[serde(rename = "daysBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_before: Option<i64>,
    #[serde(rename = "repeatDays")]
    #[serde(default)]
    pub repeat_days: i64,
}

impl DebtRemindersSettingsUpdatePartnersResponse {
    pub fn builder() -> DebtRemindersSettingsUpdatePartnersResponseBuilder {
        <DebtRemindersSettingsUpdatePartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtRemindersSettingsUpdatePartnersResponseBuilder {
    days_before: Option<i64>,
    repeat_days: Option<i64>,
}

impl DebtRemindersSettingsUpdatePartnersResponseBuilder {
    pub fn days_before(mut self, value: i64) -> Self {
        self.days_before = Some(value);
        self
    }

    pub fn repeat_days(mut self, value: i64) -> Self {
        self.repeat_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DebtRemindersSettingsUpdatePartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`repeat_days`](DebtRemindersSettingsUpdatePartnersResponseBuilder::repeat_days)
    pub fn build(self) -> Result<DebtRemindersSettingsUpdatePartnersResponse, BuildError> {
        Ok(DebtRemindersSettingsUpdatePartnersResponse {
            days_before: self.days_before,
            repeat_days: self
                .repeat_days
                .ok_or_else(|| BuildError::missing_field("repeat_days"))?,
        })
    }
}
