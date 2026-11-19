pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Us941GetDeclarationsResponse {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub year: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub quarter: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub line1: f64,
    #[serde(default)]
    pub line2: String,
    #[serde(default)]
    pub line3: String,
    #[serde(default)]
    pub line4: bool,
    #[serde(default)]
    pub line5a: Us941GetDeclarationsResponseLine5A,
    #[serde(default)]
    pub line5c: Us941GetDeclarationsResponseLine5C,
    #[serde(default)]
    pub line5d: Us941GetDeclarationsResponseLine5D,
    #[serde(default)]
    pub line5e: String,
    #[serde(default)]
    pub line6: String,
    #[serde(default)]
    pub line7: String,
    #[serde(default)]
    pub line10: String,
    #[serde(default)]
    pub line12: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line13: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line14: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line15: Option<String>,
    #[serde(default)]
    pub line16: Us941GetDeclarationsResponseLine16,
    #[serde(rename = "scheduleB")]
    #[serde(default)]
    pub schedule_b: Vec<Us941GetDeclarationsResponseScheduleBItem>,
}

impl Us941GetDeclarationsResponse {
    pub fn builder() -> Us941GetDeclarationsResponseBuilder {
        <Us941GetDeclarationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct Us941GetDeclarationsResponseBuilder {
    year: Option<f64>,
    quarter: Option<f64>,
    line1: Option<f64>,
    line2: Option<String>,
    line3: Option<String>,
    line4: Option<bool>,
    line5a: Option<Us941GetDeclarationsResponseLine5A>,
    line5c: Option<Us941GetDeclarationsResponseLine5C>,
    line5d: Option<Us941GetDeclarationsResponseLine5D>,
    line5e: Option<String>,
    line6: Option<String>,
    line7: Option<String>,
    line10: Option<String>,
    line12: Option<String>,
    line13: Option<String>,
    line14: Option<String>,
    line15: Option<String>,
    line16: Option<Us941GetDeclarationsResponseLine16>,
    schedule_b: Option<Vec<Us941GetDeclarationsResponseScheduleBItem>>,
}

impl Us941GetDeclarationsResponseBuilder {
    pub fn year(mut self, value: f64) -> Self {
        self.year = Some(value);
        self
    }

    pub fn quarter(mut self, value: f64) -> Self {
        self.quarter = Some(value);
        self
    }

    pub fn line1(mut self, value: f64) -> Self {
        self.line1 = Some(value);
        self
    }

    pub fn line2(mut self, value: impl Into<String>) -> Self {
        self.line2 = Some(value.into());
        self
    }

    pub fn line3(mut self, value: impl Into<String>) -> Self {
        self.line3 = Some(value.into());
        self
    }

    pub fn line4(mut self, value: bool) -> Self {
        self.line4 = Some(value);
        self
    }

    pub fn line5a(mut self, value: Us941GetDeclarationsResponseLine5A) -> Self {
        self.line5a = Some(value);
        self
    }

    pub fn line5c(mut self, value: Us941GetDeclarationsResponseLine5C) -> Self {
        self.line5c = Some(value);
        self
    }

    pub fn line5d(mut self, value: Us941GetDeclarationsResponseLine5D) -> Self {
        self.line5d = Some(value);
        self
    }

    pub fn line5e(mut self, value: impl Into<String>) -> Self {
        self.line5e = Some(value.into());
        self
    }

    pub fn line6(mut self, value: impl Into<String>) -> Self {
        self.line6 = Some(value.into());
        self
    }

    pub fn line7(mut self, value: impl Into<String>) -> Self {
        self.line7 = Some(value.into());
        self
    }

    pub fn line10(mut self, value: impl Into<String>) -> Self {
        self.line10 = Some(value.into());
        self
    }

    pub fn line12(mut self, value: impl Into<String>) -> Self {
        self.line12 = Some(value.into());
        self
    }

    pub fn line13(mut self, value: impl Into<String>) -> Self {
        self.line13 = Some(value.into());
        self
    }

    pub fn line14(mut self, value: impl Into<String>) -> Self {
        self.line14 = Some(value.into());
        self
    }

    pub fn line15(mut self, value: impl Into<String>) -> Self {
        self.line15 = Some(value.into());
        self
    }

    pub fn line16(mut self, value: Us941GetDeclarationsResponseLine16) -> Self {
        self.line16 = Some(value);
        self
    }

    pub fn schedule_b(mut self, value: Vec<Us941GetDeclarationsResponseScheduleBItem>) -> Self {
        self.schedule_b = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Us941GetDeclarationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`year`](Us941GetDeclarationsResponseBuilder::year)
    /// - [`quarter`](Us941GetDeclarationsResponseBuilder::quarter)
    /// - [`line1`](Us941GetDeclarationsResponseBuilder::line1)
    /// - [`line2`](Us941GetDeclarationsResponseBuilder::line2)
    /// - [`line3`](Us941GetDeclarationsResponseBuilder::line3)
    /// - [`line4`](Us941GetDeclarationsResponseBuilder::line4)
    /// - [`line5a`](Us941GetDeclarationsResponseBuilder::line5a)
    /// - [`line5c`](Us941GetDeclarationsResponseBuilder::line5c)
    /// - [`line5d`](Us941GetDeclarationsResponseBuilder::line5d)
    /// - [`line5e`](Us941GetDeclarationsResponseBuilder::line5e)
    /// - [`line6`](Us941GetDeclarationsResponseBuilder::line6)
    /// - [`line7`](Us941GetDeclarationsResponseBuilder::line7)
    /// - [`line10`](Us941GetDeclarationsResponseBuilder::line10)
    /// - [`line12`](Us941GetDeclarationsResponseBuilder::line12)
    /// - [`line16`](Us941GetDeclarationsResponseBuilder::line16)
    /// - [`schedule_b`](Us941GetDeclarationsResponseBuilder::schedule_b)
    pub fn build(self) -> Result<Us941GetDeclarationsResponse, BuildError> {
        Ok(Us941GetDeclarationsResponse {
            year: self.year.ok_or_else(|| BuildError::missing_field("year"))?,
            quarter: self
                .quarter
                .ok_or_else(|| BuildError::missing_field("quarter"))?,
            line1: self
                .line1
                .ok_or_else(|| BuildError::missing_field("line1"))?,
            line2: self
                .line2
                .ok_or_else(|| BuildError::missing_field("line2"))?,
            line3: self
                .line3
                .ok_or_else(|| BuildError::missing_field("line3"))?,
            line4: self
                .line4
                .ok_or_else(|| BuildError::missing_field("line4"))?,
            line5a: self
                .line5a
                .ok_or_else(|| BuildError::missing_field("line5a"))?,
            line5c: self
                .line5c
                .ok_or_else(|| BuildError::missing_field("line5c"))?,
            line5d: self
                .line5d
                .ok_or_else(|| BuildError::missing_field("line5d"))?,
            line5e: self
                .line5e
                .ok_or_else(|| BuildError::missing_field("line5e"))?,
            line6: self
                .line6
                .ok_or_else(|| BuildError::missing_field("line6"))?,
            line7: self
                .line7
                .ok_or_else(|| BuildError::missing_field("line7"))?,
            line10: self
                .line10
                .ok_or_else(|| BuildError::missing_field("line10"))?,
            line12: self
                .line12
                .ok_or_else(|| BuildError::missing_field("line12"))?,
            line13: self.line13,
            line14: self.line14,
            line15: self.line15,
            line16: self
                .line16
                .ok_or_else(|| BuildError::missing_field("line16"))?,
            schedule_b: self
                .schedule_b
                .ok_or_else(|| BuildError::missing_field("schedule_b"))?,
        })
    }
}
