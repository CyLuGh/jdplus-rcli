use crate::jdplus_main_ws_v1::RegressionVariableDto;
use crate::models::parameter::Parameter;
use crate::models::variable_type::VariableType;
use extendr_api::prelude::*;
use std::collections::HashMap;

#[extendr]
#[derive(Debug)]
pub struct RegressionVariable {
    pub name: String,
    pub variable_type: VariableType,
    pub coefficients: Vec<Parameter>,
    pub metadata: HashMap<String, String>,
}

impl TryFrom<RegressionVariableDto> for RegressionVariable {
    type Error = String;

    fn try_from(dto: RegressionVariableDto) -> Result<Self, Self::Error> {
        Ok(Self {
            name: dto.name,
            variable_type: VariableType::from(dto.var_type),
            coefficients: dto
                .coefficients
                .into_iter()
                .map(Parameter::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            metadata: dto.metadata,
        })
    }
}
