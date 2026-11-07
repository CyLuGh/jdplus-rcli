use extendr_api::prelude::*;
use crate::models::time_selector::TimeSelector;

#[extendr]
pub struct BasicSpec {
    pub span: TimeSelector,
    pub preliminary_check: bool,
    pub annual_frequency: i32
}