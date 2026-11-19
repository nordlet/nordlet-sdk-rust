pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemDaysItem {
    #[serde(default)]
    pub date: NaiveDate,
    #[serde(default)]
    pub hours: String,
}

impl UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemDaysItem {
    pub fn builder() -> UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemDaysItemBuilder {
        <UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemDaysItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemDaysItemBuilder {
    date: Option<NaiveDate>,
    hours: Option<String>,
}

impl UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemDaysItemBuilder {
    pub fn date(mut self, value: NaiveDate) -> Self {
        self.date = Some(value);
        self
    }

    pub fn hours(mut self, value: impl Into<String>) -> Self {
        self.hours = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemDaysItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date`](UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemDaysItemBuilder::date)
    /// - [`hours`](UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemDaysItemBuilder::hours)
    pub fn build(
        self,
    ) -> Result<UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemDaysItem, BuildError> {
        Ok(
            UsWorkweeksListDeclarationsResponseEmployeesItemWeeksItemDaysItem {
                date: self.date.ok_or_else(|| BuildError::missing_field("date"))?,
                hours: self
                    .hours
                    .ok_or_else(|| BuildError::missing_field("hours"))?,
            },
        )
    }
}
