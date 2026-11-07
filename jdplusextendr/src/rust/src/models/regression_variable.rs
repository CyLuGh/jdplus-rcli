use std::collections::HashMap;
use extendr_api::prelude::*;
use crate::models::parameter::Parameter;
use crate::models::variable_type::VariableType;

#[extendr]
pub struct RegressionVariable {
    pub name: String,
    pub variable_type: VariableType,
    pub coefficients : Vec<Parameter>,
    pub metadata: HashMap<String, String>
}