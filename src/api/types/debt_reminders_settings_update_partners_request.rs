pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DebtRemindersSettingsUpdatePartnersRequest {
    #[serde(rename = "daysBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_before: Option<i64>,
    #[serde(rename = "repeatDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_days: Option<i64>,
}

impl DebtRemindersSettingsUpdatePartnersRequest {
    pub fn builder() -> DebtRemindersSettingsUpdatePartnersRequestBuilder {
        <DebtRemindersSettingsUpdatePartnersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DebtRemindersSettingsUpdatePartnersRequestBuilder {
    days_before: Option<i64>,
    repeat_days: Option<i64>,
}

impl DebtRemindersSettingsUpdatePartnersRequestBuilder {
    pub fn days_before(mut self, value: i64) -> Self {
        self.days_before = Some(value);
        self
    }

    pub fn repeat_days(mut self, value: i64) -> Self {
        self.repeat_days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DebtRemindersSettingsUpdatePartnersRequest`].
    pub fn build(self) -> Result<DebtRemindersSettingsUpdatePartnersRequest, BuildError> {
        Ok(DebtRemindersSettingsUpdatePartnersRequest {
            days_before: self.days_before,
            repeat_days: self.repeat_days,
        })
    }
}
