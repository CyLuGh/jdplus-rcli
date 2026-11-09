use extendr_api::prelude::*;
use crate::models::length_of_period::LengthOfPeriod;
use crate::models::regression_variable::RegressionVariable;
use crate::models::specs::SarimaSpec;
use crate::models::tsdata::TsData;

#[extendr]
pub struct Description {
    pub series: TsData,
    pub log : bool,
    pub preadjustment: LengthOfPeriod,
    pub variables: Vec<RegressionVariable>,
    pub arima: SarimaSpec
}