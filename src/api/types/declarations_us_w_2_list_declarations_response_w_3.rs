pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UsW2ListDeclarationsResponseW3 {
    #[serde(rename = "boxC")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub box_c: f64,
    #[serde(rename = "boxE")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_e: Option<String>,
    #[serde(rename = "boxF")]
    #[serde(default)]
    pub box_f: String,
    #[serde(default)]
    pub box1: String,
    #[serde(default)]
    pub box2: String,
    #[serde(default)]
    pub box3: String,
    #[serde(default)]
    pub box4: String,
    #[serde(default)]
    pub box5: String,
    #[serde(default)]
    pub box6: String,
    #[serde(default)]
    pub box16: String,
    #[serde(default)]
    pub box17: String,
}

impl UsW2ListDeclarationsResponseW3 {
    pub fn builder() -> UsW2ListDeclarationsResponseW3Builder {
        <UsW2ListDeclarationsResponseW3Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsW2ListDeclarationsResponseW3Builder {
    box_c: Option<f64>,
    box_e: Option<String>,
    box_f: Option<String>,
    box1: Option<String>,
    box2: Option<String>,
    box3: Option<String>,
    box4: Option<String>,
    box5: Option<String>,
    box6: Option<String>,
    box16: Option<String>,
    box17: Option<String>,
}

impl UsW2ListDeclarationsResponseW3Builder {
    pub fn box_c(mut self, value: f64) -> Self {
        self.box_c = Some(value);
        self
    }

    pub fn box_e(mut self, value: impl Into<String>) -> Self {
        self.box_e = Some(value.into());
        self
    }

    pub fn box_f(mut self, value: impl Into<String>) -> Self {
        self.box_f = Some(value.into());
        self
    }

    pub fn box1(mut self, value: impl Into<String>) -> Self {
        self.box1 = Some(value.into());
        self
    }

    pub fn box2(mut self, value: impl Into<String>) -> Self {
        self.box2 = Some(value.into());
        self
    }

    pub fn box3(mut self, value: impl Into<String>) -> Self {
        self.box3 = Some(value.into());
        self
    }

    pub fn box4(mut self, value: impl Into<String>) -> Self {
        self.box4 = Some(value.into());
        self
    }

    pub fn box5(mut self, value: impl Into<String>) -> Self {
        self.box5 = Some(value.into());
        self
    }

    pub fn box6(mut self, value: impl Into<String>) -> Self {
        self.box6 = Some(value.into());
        self
    }

    pub fn box16(mut self, value: impl Into<String>) -> Self {
        self.box16 = Some(value.into());
        self
    }

    pub fn box17(mut self, value: impl Into<String>) -> Self {
        self.box17 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsW2ListDeclarationsResponseW3`].
    /// This method will fail if any of the following fields are not set:
    /// - [`box_c`](UsW2ListDeclarationsResponseW3Builder::box_c)
    /// - [`box_f`](UsW2ListDeclarationsResponseW3Builder::box_f)
    /// - [`box1`](UsW2ListDeclarationsResponseW3Builder::box1)
    /// - [`box2`](UsW2ListDeclarationsResponseW3Builder::box2)
    /// - [`box3`](UsW2ListDeclarationsResponseW3Builder::box3)
    /// - [`box4`](UsW2ListDeclarationsResponseW3Builder::box4)
    /// - [`box5`](UsW2ListDeclarationsResponseW3Builder::box5)
    /// - [`box6`](UsW2ListDeclarationsResponseW3Builder::box6)
    /// - [`box16`](UsW2ListDeclarationsResponseW3Builder::box16)
    /// - [`box17`](UsW2ListDeclarationsResponseW3Builder::box17)
    pub fn build(self) -> Result<UsW2ListDeclarationsResponseW3, BuildError> {
        Ok(UsW2ListDeclarationsResponseW3 {
            box_c: self
                .box_c
                .ok_or_else(|| BuildError::missing_field("box_c"))?,
            box_e: self.box_e,
            box_f: self
                .box_f
                .ok_or_else(|| BuildError::missing_field("box_f"))?,
            box1: self.box1.ok_or_else(|| BuildError::missing_field("box1"))?,
            box2: self.box2.ok_or_else(|| BuildError::missing_field("box2"))?,
            box3: self.box3.ok_or_else(|| BuildError::missing_field("box3"))?,
            box4: self.box4.ok_or_else(|| BuildError::missing_field("box4"))?,
            box5: self.box5.ok_or_else(|| BuildError::missing_field("box5"))?,
            box6: self.box6.ok_or_else(|| BuildError::missing_field("box6"))?,
            box16: self
                .box16
                .ok_or_else(|| BuildError::missing_field("box16"))?,
            box17: self
                .box17
                .ok_or_else(|| BuildError::missing_field("box17"))?,
        })
    }
}
