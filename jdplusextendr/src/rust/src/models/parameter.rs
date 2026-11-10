use crate::jdplus_main_ws_v1::ParameterDto;
use extendr_api::prelude::*;

#[extendr]
#[derive(Debug, Clone)]
pub struct Parameter {
    pub value: f64,
    pub parameter_type: ParameterType,
    pub description: String,
}

impl TryFrom<ParameterDto> for Parameter {
    type Error = String;

    fn try_from(dto: ParameterDto) -> Result<Self, Self::Error> {
        Ok(Self {
            value: dto.value,
            description: dto.description,
            parameter_type: ParameterType::from(dto.r#type),
        })
    }
}

#[extendr]
#[derive(Debug,Clone)]
pub enum ParameterType {
    /// Similar to null
    Unused = 0,
    /// Parameter is used but not defined (to be estimated)
    Undefined = 1,
    Fixed = 2,
    Initial = 3,
    Estimated = 4,
}

impl From<i32> for ParameterType {
    fn from(value: i32) -> Self {
        match value {
            0 => ParameterType::Unused,
            1 => ParameterType::Undefined,
            2 => ParameterType::Fixed,
            3 => ParameterType::Initial,
            4 => ParameterType::Estimated,
            _ => ParameterType::Unused,
        }
    }
}
