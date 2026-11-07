use extendr_api::prelude::*;
use crate::models::parameter::Parameter;

#[extendr]
pub struct SarimaSpec {
    pub period: i32,
    pub phi: Vec<Parameter>,
    pub d: i32,
    pub theta: Vec<Parameter>,
    pub bphi: Vec<Parameter>,
    pub bd: i32,
    pub btheta: Vec<Parameter>,
}