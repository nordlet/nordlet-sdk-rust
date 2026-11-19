pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItem {
    #[serde(default)]
    pub start: String,
    #[serde(default)]
    pub end: String,
    #[serde(default)]
    pub hours: String,
    #[serde(rename = "overtimeHours")]
    #[serde(default)]
    pub overtime_hours: String,
    #[serde(default)]
    pub days: Vec<UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemDaysItem>,
}

impl UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItem {
    pub fn builder() -> UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemBuilder {
        <UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemBuilder {
    start: Option<String>,
    end: Option<String>,
    hours: Option<String>,
    overtime_hours: Option<String>,
    days: Option<Vec<UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemDaysItem>>,
}

impl UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemBuilder {
    pub fn start(mut self, value: impl Into<String>) -> Self {
        self.start = Some(value.into());
        self
    }

    pub fn end(mut self, value: impl Into<String>) -> Self {
        self.end = Some(value.into());
        self
    }

    pub fn hours(mut self, value: impl Into<String>) -> Self {
        self.hours = Some(value.into());
        self
    }

    pub fn overtime_hours(mut self, value: impl Into<String>) -> Self {
        self.overtime_hours = Some(value.into());
        self
    }

    pub fn days(
        mut self,
        value: Vec<UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemDaysItem>,
    ) -> Self {
        self.days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start`](UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemBuilder::start)
    /// - [`end`](UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemBuilder::end)
    /// - [`hours`](UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemBuilder::hours)
    /// - [`overtime_hours`](UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemBuilder::overtime_hours)
    /// - [`days`](UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemBuilder::days)
    pub fn build(
        self,
    ) -> Result<UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItem, BuildError> {
        Ok(UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItem {
            start: self
                .start
                .ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
            hours: self
                .hours
                .ok_or_else(|| BuildError::missing_field("hours"))?,
            overtime_hours: self
                .overtime_hours
                .ok_or_else(|| BuildError::missing_field("overtime_hours"))?,
            days: self.days.ok_or_else(|| BuildError::missing_field("days"))?,
        })
    }
}
