use extendr_api::prelude::*;

#[extendr]
pub enum VariableType {
    Unspecified = 0,
    Mean = 1,
    Td = 10,
    Lp = 11,
    Easter = 12,
    Ao = 20,
    Ls = 21,
    Tc = 22,
    So = 23,
    Outlier = 29,
    Iv = 30,
    Ramp = 40
}

impl From<i32> for VariableType {
    fn from(item: i32) -> Self {
        match item {
            0 => VariableType::Unspecified,
            1 => VariableType::Mean,
            10 => VariableType::Td,
            11 => VariableType::Lp,
            12 => VariableType::Easter,
            20 => VariableType::Ao,
            21 => VariableType::Ls,
            22 => VariableType::Tc,
            23 => VariableType::So,
            29 => VariableType::Outlier,
            30 => VariableType::Iv,
            40 => VariableType::Ramp,
            _ => unreachable!()
        }
    }
}