use crate::jdplus_main_ws_v1::reg_arima_model_dto::{
    DescriptionDto, EstimationDto, MissingEstimationDto,
};
use crate::jdplus_main_ws_v1::{DiagnosticsDto, ParametersEstimationDto, RegArimaModelDto};
use crate::models::length_of_period::LengthOfPeriod;
use crate::models::likelihood_statistics::LikelihoodStatistics;
use crate::models::matrix::Matrix;
use crate::models::regression_variable::RegressionVariable;
use crate::models::specs::SarimaSpec;
use crate::models::statistical_test::StatisticalTest;
use crate::models::tsdata::TsData;
use extendr_api::prelude::*;
use std::collections::HashMap;

#[extendr]
#[derive(Debug)]
pub struct RegArimaModel {
    pub description: Description,
    pub estimation: Estimation,
    pub diagnostics: Diagnostics,
}

impl TryFrom<RegArimaModelDto> for RegArimaModel {
    type Error = String;

    fn try_from(dto: RegArimaModelDto) -> Result<Self, Self::Error> {
        Ok(Self {
            description: Description::try_from(dto.description.unwrap())?,
            estimation: Estimation::try_from(dto.estimation.unwrap())?,
            diagnostics: Diagnostics::from(dto.diagnostics.unwrap()),
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct Description {
    pub series: TsData,
    pub log: bool,
    pub preadjustment: LengthOfPeriod,
    pub variables: Vec<RegressionVariable>,
    pub arima: SarimaSpec,
}

impl TryFrom<DescriptionDto> for Description {
    type Error = String;

    fn try_from(dto: DescriptionDto) -> Result<Self, Self::Error> {
        Ok(Self {
            series: TsData::try_from(dto.series.unwrap())?,
            log: dto.log,
            preadjustment: LengthOfPeriod::from(dto.preadjustment),
            variables: dto
                .variables
                .into_iter()
                .map(RegressionVariable::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            arima: SarimaSpec::try_from(dto.arima.unwrap())?,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct Estimation {
    pub y: Vec<f64>,
    pub x: Matrix,
    pub b: Vec<f64>,
    pub b_covariance: Matrix,
    pub parameters: ParametersEstimation,
    pub likelihood: LikelihoodStatistics,
    pub residuals: Vec<f64>,
    pub missings: Vec<MissingEstimation>,
}

impl TryFrom<EstimationDto> for Estimation {
    type Error = String;
    fn try_from(dto: EstimationDto) -> Result<Self, Self::Error> {
        Ok(Self {
            y: dto.y,
            x: Matrix::from(dto.x.unwrap()),
            b: dto.b,
            b_covariance: Matrix::from(dto.bcovariance.unwrap()),
            parameters: ParametersEstimation::try_from(dto.parameters.unwrap())?,
            likelihood: LikelihoodStatistics::from(dto.likelihood.unwrap()),
            residuals: dto.residuals,
            missings: dto
                .missings
                .into_iter()
                .map(MissingEstimation::from)
                .collect(),
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct ParametersEstimation {
    pub value: Vec<f64>,
    pub score: Vec<f64>,
    pub covariance: Matrix,
    pub description: Option<String>,
}

impl TryFrom<ParametersEstimationDto> for ParametersEstimation {
    type Error = String;

    fn try_from(dto: ParametersEstimationDto) -> Result<Self, Self::Error> {
        Ok(Self {
            value: dto.value,
            score: dto.score,
            covariance: Matrix::from(dto.covariance.unwrap()),
            description: dto.description,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct MissingEstimation {
    pub position: i32,
    pub value: f64,
    pub stdev: f64,
}

impl From<MissingEstimationDto> for MissingEstimation {
    fn from(dto: MissingEstimationDto) -> Self {
        Self {
            position: dto.position,
            value: dto.value,
            stdev: dto.stde,
        }
    }
}

#[extendr]
#[derive(Debug)]
pub struct Diagnostics {
    pub residuals_tests: HashMap<String, StatisticalTest>,
}

impl From<DiagnosticsDto> for Diagnostics {
    fn from(dto: DiagnosticsDto) -> Self {
        Self {
            residuals_tests: dto
                .residuals_tests
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}
