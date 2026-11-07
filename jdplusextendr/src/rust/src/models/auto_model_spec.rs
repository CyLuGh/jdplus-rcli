use extendr_api::prelude::*;

#[extendr]
pub struct AutoModelSpec {
    pub enabled: bool,
    pub cancel: f64,
    pub ub1: f64,
    pub ub2: f64,
    pub pcr: f64,
    pub pc: f64,
    pub tsig: f64,
    pub accept_def: bool,
    pub ami_compare: bool,
}
