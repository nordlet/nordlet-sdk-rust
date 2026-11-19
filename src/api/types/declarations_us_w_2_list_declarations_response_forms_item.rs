pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsW2ListDeclarationsResponseFormsItem {
    #[serde(rename = "employeeId")]
    #[serde(default)]
    pub employee_id: String,
    #[serde(rename = "boxA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_a: Option<String>,
    #[serde(rename = "boxE")]
    #[serde(default)]
    pub box_e: UsW2ListDeclarationsResponseFormsItemBoxE,
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
    pub states: Vec<UsW2ListDeclarationsResponseFormsItemStatesItem>,
}

impl UsW2ListDeclarationsResponseFormsItem {
    pub fn builder() -> UsW2ListDeclarationsResponseFormsItemBuilder {
        <UsW2ListDeclarationsResponseFormsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsW2ListDeclarationsResponseFormsItemBuilder {
    employee_id: Option<String>,
    box_a: Option<String>,
    box_e: Option<UsW2ListDeclarationsResponseFormsItemBoxE>,
    box1: Option<String>,
    box2: Option<String>,
    box3: Option<String>,
    box4: Option<String>,
    box5: Option<String>,
    box6: Option<String>,
    states: Option<Vec<UsW2ListDeclarationsResponseFormsItemStatesItem>>,
}

impl UsW2ListDeclarationsResponseFormsItemBuilder {
    pub fn employee_id(mut self, value: impl Into<String>) -> Self {
        self.employee_id = Some(value.into());
        self
    }

    pub fn box_a(mut self, value: impl Into<String>) -> Self {
        self.box_a = Some(value.into());
        self
    }

    pub fn box_e(mut self, value: UsW2ListDeclarationsResponseFormsItemBoxE) -> Self {
        self.box_e = Some(value);
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

    pub fn states(mut self, value: Vec<UsW2ListDeclarationsResponseFormsItemStatesItem>) -> Self {
        self.states = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsW2ListDeclarationsResponseFormsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`employee_id`](UsW2ListDeclarationsResponseFormsItemBuilder::employee_id)
    /// - [`box_e`](UsW2ListDeclarationsResponseFormsItemBuilder::box_e)
    /// - [`box1`](UsW2ListDeclarationsResponseFormsItemBuilder::box1)
    /// - [`box2`](UsW2ListDeclarationsResponseFormsItemBuilder::box2)
    /// - [`box3`](UsW2ListDeclarationsResponseFormsItemBuilder::box3)
    /// - [`box4`](UsW2ListDeclarationsResponseFormsItemBuilder::box4)
    /// - [`box5`](UsW2ListDeclarationsResponseFormsItemBuilder::box5)
    /// - [`box6`](UsW2ListDeclarationsResponseFormsItemBuilder::box6)
    /// - [`states`](UsW2ListDeclarationsResponseFormsItemBuilder::states)
    pub fn build(self) -> Result<UsW2ListDeclarationsResponseFormsItem, BuildError> {
        Ok(UsW2ListDeclarationsResponseFormsItem {
            employee_id: self
                .employee_id
                .ok_or_else(|| BuildError::missing_field("employee_id"))?,
            box_a: self.box_a,
            box_e: self
                .box_e
                .ok_or_else(|| BuildError::missing_field("box_e"))?,
            box1: self.box1.ok_or_else(|| BuildError::missing_field("box1"))?,
            box2: self.box2.ok_or_else(|| BuildError::missing_field("box2"))?,
            box3: self.box3.ok_or_else(|| BuildError::missing_field("box3"))?,
            box4: self.box4.ok_or_else(|| BuildError::missing_field("box4"))?,
            box5: self.box5.ok_or_else(|| BuildError::missing_field("box5"))?,
            box6: self.box6.ok_or_else(|| BuildError::missing_field("box6"))?,
            states: self
                .states
                .ok_or_else(|| BuildError::missing_field("states"))?,
        })
    }
}
