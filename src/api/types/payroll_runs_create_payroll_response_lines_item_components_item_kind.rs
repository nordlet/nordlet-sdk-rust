pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunsCreatePayrollResponseLinesItemComponentsItemKind {
    Allowance,
    EmployeeTax,
    EmployeeContribution,
    EmployerContribution,
    EmployerPayment,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RunsCreatePayrollResponseLinesItemComponentsItemKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Allowance => serializer.serialize_str("allowance"),
            Self::EmployeeTax => serializer.serialize_str("employee_tax"),
            Self::EmployeeContribution => serializer.serialize_str("employee_contribution"),
            Self::EmployerContribution => serializer.serialize_str("employer_contribution"),
            Self::EmployerPayment => serializer.serialize_str("employer_payment"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RunsCreatePayrollResponseLinesItemComponentsItemKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "allowance" => Ok(Self::Allowance),
            "employee_tax" => Ok(Self::EmployeeTax),
            "employee_contribution" => Ok(Self::EmployeeContribution),
            "employer_contribution" => Ok(Self::EmployerContribution),
            "employer_payment" => Ok(Self::EmployerPayment),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RunsCreatePayrollResponseLinesItemComponentsItemKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allowance => write!(f, "allowance"),
            Self::EmployeeTax => write!(f, "employee_tax"),
            Self::EmployeeContribution => write!(f, "employee_contribution"),
            Self::EmployerContribution => write!(f, "employer_contribution"),
            Self::EmployerPayment => write!(f, "employer_payment"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
