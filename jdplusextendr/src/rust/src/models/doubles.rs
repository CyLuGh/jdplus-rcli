use extendr_api::prelude::*;

#[extendr]
pub struct Doubles{
    pub name: String,
    pub values: Vec<f64>
}