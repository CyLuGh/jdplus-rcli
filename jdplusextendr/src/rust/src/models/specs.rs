use crate::jdplus_main_ws_v1::intervention_variable_dto::SequenceDto;
use crate::jdplus_main_ws_v1::{
    AutoModelSpecDto, BasicSpecDto, EasterSpecDto, EstimateSpecDto, InterventionVariableDto,
    OutlierDto, OutlierSpecDto, RampDto, RegressionSpecDto, SarimaSpecDto, TradingDaysSpecDto,
    TramoSpecDto, TransformSpecDto, TsVariableDto,
};
use crate::models::easter_type::EasterType;
use crate::models::length_of_period::LengthOfPeriod;
use crate::models::parameter::Parameter;
use crate::models::time_selector::TimeSelector;
use crate::models::trading_days::{AutomaticTradingDays, TradingDays, TradingDaysTest};
use crate::models::transformation::Transformation;
use chrono::NaiveDate;
use extendr_api::prelude::*;
use std::collections::HashMap;

#[extendr]
#[derive(Debug)]
pub struct BasicSpec {
    pub span: TimeSelector,
    pub preliminary_check: bool,
    pub annual_frequency: i32,
}

impl TryFrom<BasicSpecDto> for BasicSpec {
    type Error = String;

    fn try_from(dto: BasicSpecDto) -> Result<Self, Self::Error> {
        Ok(Self {
            preliminary_check: dto.preliminary_check,
            annual_frequency: dto.annual_frequency,
            span: TimeSelector::try_from(dto.span.unwrap())?,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct TransformSpec {
    pub transformation: Transformation,
    pub fct: f64,
    pub adjust: LengthOfPeriod,
    pub outliers_correction: bool,
}

impl TryFrom<TransformSpecDto> for TransformSpec {
    type Error = String;

    fn try_from(dto: TransformSpecDto) -> Result<Self, Self::Error> {
        Ok(Self {
            transformation: Transformation::from(dto.transformation),
            fct: dto.fct,
            adjust: LengthOfPeriod::from(dto.adjust),
            outliers_correction: dto.outliers_correction,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct OutlierSpec {
    pub enabled: bool,
    pub span: TimeSelector,
    pub ao: bool,
    pub ls: bool,
    pub tc: bool,
    pub so: bool,
    pub va: f64,
    pub tcrate: f64,
    pub ml: bool,
}

impl TryFrom<OutlierSpecDto> for OutlierSpec {
    type Error = String;

    fn try_from(dto: OutlierSpecDto) -> Result<Self, Self::Error> {
        Ok(Self {
            enabled: dto.enabled,
            span: TimeSelector::try_from(dto.span.unwrap())?,
            ao: dto.ao,
            ls: dto.ls,
            tc: dto.tc,
            so: dto.so,
            va: dto.va,
            tcrate: dto.tcrate,
            ml: dto.ml,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct SarimaSpec {
    pub period: i32,
    pub phi: Vec<Parameter>,
    pub d: i32,
    pub theta: Vec<Parameter>,
    pub bphi: Vec<Parameter>,
    pub bd: i32,
    pub btheta: Vec<Parameter>,
}

impl TryFrom<SarimaSpecDto> for SarimaSpec {
    type Error = String;

    fn try_from(dto: SarimaSpecDto) -> Result<Self, Self::Error> {
        Ok(Self {
            period: dto.period,
            phi: dto
                .phi
                .into_iter()
                .map(Parameter::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            d: dto.d,
            theta: dto
                .theta
                .into_iter()
                .map(Parameter::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            bphi: dto
                .bphi
                .into_iter()
                .map(Parameter::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            bd: dto.bd,
            btheta: dto
                .btheta
                .into_iter()
                .map(Parameter::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

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

impl From<AutoModelSpecDto> for AutoModelSpec {
    fn from(dto: AutoModelSpecDto) -> Self {
        Self {
            enabled: dto.enabled,
            cancel: dto.cancel,
            ub1: dto.ub1,
            ub2: dto.ub2,
            pcr: dto.pcr,
            pc: dto.pc,
            tsig: dto.tsig,
            accept_def: dto.accept_def,
            ami_compare: dto.ami_compare,
        }
    }
}

#[extendr]
#[derive(Debug)]
pub struct RegressionSpec {
    pub mean: Option<Parameter>,
    pub check_mean: bool,
    pub td: TradingDaysSpec,
    pub easter: EasterSpec,
    pub outliers: Vec<Outlier>,
    pub users: Vec<TsVariable>,
    pub interventions: Vec<InterventionVariable>,
    pub ramps: Vec<Ramp>,
}

impl TryFrom<RegressionSpecDto> for RegressionSpec {
    type Error = String;

    fn try_from(dto: RegressionSpecDto) -> Result<Self, Self::Error> {
        Ok(Self {
            mean: dto.mean.map(Parameter::try_from).transpose()?,
            check_mean: dto.check_mean,
            td: TradingDaysSpec::try_from(dto.td.unwrap())?,
            easter: EasterSpec::try_from(dto.easter.unwrap())?,
            outliers: dto
                .outliers
                .into_iter()
                .map(Outlier::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            users: dto
                .users
                .into_iter()
                .map(TsVariable::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            interventions: dto
                .interventions
                .into_iter()
                .map(InterventionVariable::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            ramps: dto
                .ramps
                .into_iter()
                .map(Ramp::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct TradingDaysSpec {
    pub td: TradingDays,
    pub lp: LengthOfPeriod,
    pub holidays: String,
    pub users: Vec<String>,
    pub w: i32,
    pub test: TradingDaysTest,
    pub auto: AutomaticTradingDays,
    pub p_test: f64,
    pub auto_adjust: bool,
    pub td_coefficients: Vec<Parameter>,
    pub lp_coefficient: Option<Parameter>,
}

impl TryFrom<TradingDaysSpecDto> for TradingDaysSpec {
    type Error = String;

    fn try_from(dto: TradingDaysSpecDto) -> Result<Self, Self::Error> {
        Ok(Self {
            td: TradingDays::from(dto.td),
            lp: LengthOfPeriod::from(dto.lp),
            holidays: dto.holidays,
            users: dto.users,
            w: dto.w,
            test: TradingDaysTest::from(dto.test),
            auto: AutomaticTradingDays::from(dto.auto),
            p_test: dto.ptest,
            auto_adjust: dto.auto_adjust,
            td_coefficients: dto
                .tdcoefficients
                .into_iter()
                .map(Parameter::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            lp_coefficient: dto.lpcoefficient.map(Parameter::try_from).transpose()?,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct EasterSpec {
    pub easter_type: EasterType,
    pub duration: i32,
    pub julian: bool,
    pub test: bool,
    pub coefficient: Option<Parameter>,
}

impl TryFrom<EasterSpecDto> for EasterSpec {
    type Error = String;

    fn try_from(dto: EasterSpecDto) -> Result<Self, Self::Error> {
        Ok(Self {
            easter_type: EasterType::from(dto.r#type),
            duration: dto.duration,
            julian: dto.julian,
            test: dto.test,
            coefficient: dto.coefficient.map(Parameter::try_from).transpose()?,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct Outlier {
    pub name: String,
    pub code: String,
    pub position: NaiveDate,
    pub coefficient: Parameter,
    pub metadata: HashMap<String, String>,
}

impl TryFrom<OutlierDto> for Outlier {
    type Error = String;

    fn try_from(dto: OutlierDto) -> Result<Self, Self::Error> {
        Ok(Self {
            name: dto.name,
            code: dto.code,
            position: NaiveDate::from(dto.position.unwrap()),
            coefficient: Parameter::try_from(dto.coefficient.unwrap())?,
            metadata: dto.metadata,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct TsVariable {
    pub name: String,
    pub id: String,
    pub lag: i32,
    pub coefficient: Parameter,
    pub metadata: HashMap<String, String>,
}

impl TryFrom<TsVariableDto> for TsVariable {
    type Error = String;

    fn try_from(dto: TsVariableDto) -> Result<Self, Self::Error> {
        Ok(Self {
            name: dto.name,
            id: dto.id,
            lag: dto.lag,
            coefficient: Parameter::try_from(dto.coefficient.unwrap())?,
            metadata: dto.metadata,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct InterventionVariable {
    pub name: String,
    pub sequences: Vec<Sequence>,
    pub delta: f64,
    pub seasonal_delta: f64,
    pub coefficient: Parameter,
    pub metadata: HashMap<String, String>,
}

impl TryFrom<InterventionVariableDto> for InterventionVariable {
    type Error = String;

    fn try_from(dto: InterventionVariableDto) -> Result<Self, Self::Error> {
        Ok(Self {
            name: dto.name,
            sequences: dto
                .sequences
                .into_iter()
                .map(Sequence::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            delta: dto.delta,
            seasonal_delta: dto.seasonal_delta,
            coefficient: Parameter::try_from(dto.coefficient.unwrap())?,
            metadata: dto.metadata,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct Sequence {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

impl TryFrom<SequenceDto> for Sequence {
    type Error = String;

    fn try_from(dto: SequenceDto) -> Result<Self, Self::Error> {
        Ok(Self {
            start: NaiveDate::from(dto.start.unwrap()),
            end: NaiveDate::from(dto.end.unwrap()),
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct Ramp {
    pub name: String,
    pub start: NaiveDate,
    pub end: NaiveDate,
    pub coefficient: Parameter,
    pub metadata: HashMap<String, String>,
}

impl TryFrom<RampDto> for Ramp {
    type Error = String;

    fn try_from(dto: RampDto) -> Result<Self, Self::Error> {
        Ok(Self {
            name: dto.name,
            start: NaiveDate::from(dto.start.unwrap()),
            end: NaiveDate::from(dto.end.unwrap()),
            coefficient: Parameter::try_from(dto.coefficient.unwrap())?,
            metadata: dto.metadata,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct EstimateSpec {
    pub span: TimeSelector,
    pub ml: bool,
    pub tol: f64,
    pub ubp: f64,
}

impl TryFrom<EstimateSpecDto> for EstimateSpec {
    type Error = String;

    fn try_from(dto: EstimateSpecDto) -> Result<Self, Self::Error> {
        Ok(Self {
            span: TimeSelector::try_from(dto.span.unwrap())?,
            ml: dto.ml,
            tol: dto.tol,
            ubp: dto.ubp,
        })
    }
}

#[extendr]
#[derive(Debug)]
pub struct TramoSpec {
    pub basic: BasicSpec,
    pub transform: TransformSpec,
    pub outlier: OutlierSpec,
    pub arima: SarimaSpec,
    pub regression: RegressionSpec,
    pub estimate: EstimateSpec,
}

impl TryFrom<TramoSpecDto> for TramoSpec {
    type Error = String;

    fn try_from(dto: TramoSpecDto) -> Result<Self, Self::Error> {
        Ok(Self {
            basic: BasicSpec::try_from(dto.basic.unwrap())?,
            transform: TransformSpec::try_from(dto.transform.unwrap())?,
            outlier: OutlierSpec::try_from(dto.outlier.unwrap())?,
            arima: SarimaSpec::try_from(dto.arima.unwrap())?,
            regression: RegressionSpec::try_from(dto.regression.unwrap())?,
            estimate: EstimateSpec::try_from(dto.estimate.unwrap())?,
        })
    }
}
